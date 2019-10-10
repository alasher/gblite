#![allow(dead_code)]

use memory::Memory;
use memory::MemClient;
use ppu::PPU;
use lookup::Instruction;
use registers::*;
use util;
use lookup;

use std::fmt;
use std::io;
use std::io::Write;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Copy, Clone, PartialEq)]
enum AluOp {
    Add(bool),
    Sub(bool),
    And,
    Xor,
    Or,
    Comp,
    RotateLeft(bool),
    RotateRight(bool),
    ShiftLeft,
    ShiftRight(bool),
    Swap,
    Test(u8),
    Set(u8, bool)
}

impl fmt::Display for AluOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_name = match *self {
            AluOp::Add(c) => if c { format!("AddCarry") } else { format!("Add") },
            AluOp::Sub(c) => if c { format!("SubCarry") } else { format!("Sub") },
            AluOp::And => format!("And"),
            AluOp::Xor => format!("Xor"),
            AluOp::Or => format!("Or"),
            AluOp::Comp => format!("Comp"),
            AluOp::RotateLeft(c) => if c { format!("RotateLeftCarry") } else { format!("RotateLeft") },
            AluOp::RotateRight(c) => if c { format!("RotateRightCarry") } else { format!("RotateRight") },
            AluOp::ShiftLeft => format!("ShiftLeft"),
            AluOp::ShiftRight(c) => if c { format!("ShiftRightArithmetic") } else { format!("ShiftRightLogical") },
            AluOp::Swap => format!("Swap"),
            AluOp::Test(x) => format!("TestBit{}", x),
            AluOp::Set(x, val) => format!("SetBit{}To{}", x, if val { '1' } else { '0' })
        };

        write!(f, "{}", op_name)
    }
}

pub struct CPU {
    pub regs: RegisterCache,
    pub mem: Arc<Mutex<Memory>>,
    pub ppu: PPU,
    ir_enabled: bool,
    quit: bool,
    was_zero: bool,
    half_carry: bool,
    full_carry: bool,
    step: bool,
    breaks: HashSet<u16>
}

impl CPU {
    pub fn new(mem: Arc<Mutex<Memory>>, ppu: PPU) -> CPU {
        CPU {
            regs: RegisterCache::new(),
            mem: mem,
            ppu: ppu,
            ir_enabled: true,
            quit: false,
            was_zero: false,
            half_carry: false,
            full_carry: false,
            step: false,
            breaks: HashSet::new()
        }
    }

    // Lock the memory object and return byte at the given memory address.
    fn mem_get(&self, addr: u16) -> u8 {
        let mref = self.mem.lock().unwrap();
        (*mref).get(addr, MemClient::CPU)
    }

    // Lock the memory object and set byte at the given memory address with the given value.
    fn mem_set(&mut self, val: u8, addr: u16) {
        let mut mref = self.mem.lock().unwrap();
        (*mref).set(val, addr, MemClient::CPU);
    }

    // Get the u16 value starting at $(addr), little endian.
    fn parse_u16(&self, addr: u16) -> u16 {
        util::join_u8((self.mem_get(addr), self.mem_get(addr+1)))
    }

    // Push addr from given register onto stack
    fn push(&mut self, src: Reg16) {
        self.regs.sub(Reg16::SP, 2);
        let sp_val = self.regs.get(Reg16::SP);
        let split_addr = util::split_u16(self.regs.get(src));
        self.mem_set(split_addr.0, sp_val);
        self.mem_set(split_addr.1, sp_val+1);
    }

    // Pop topmost u16 value from stack, store to given register
    fn pop(&mut self, dst: Reg16) {
        let stack_val = self.parse_u16(self.regs.get(Reg16::SP));
        self.regs.add(Reg16::SP, 2);
        self.regs.set(dst, stack_val);
    }

    // Call the value at address if flag value is set, or unset.
    fn call_flag(&mut self, flag: Flag, if_unset: bool, addr: u16) {
        let flag_val = match flag {
            Flag::Z | Flag::CY => self.regs.get_flag(flag),
            _ => panic!("CALL for flag only exists for Z and CY flags!")
        };

        if flag_val ^ if_unset {
            self.call(addr);
        }
    }

    // Push PC to stack, and jump to the jump_addr.
    fn call(&mut self, jump_addr: u16) {
        self.push(Reg16::PC);
        self.regs.set(Reg16::PC, jump_addr);
    }

    // Execute a return if given flag is set, or unset.
    fn ret_flag(&mut self, flag: Flag, if_unset: bool) {
        let flag_val = match flag {
            Flag::Z | Flag::CY => self.regs.get_flag(flag),
            _ => panic!("RET for flag only exists for Z and CY flags!")
        };

        if flag_val ^ if_unset {
            self.ret(false);
        }
    }

    // Pop the topmost address from the stack, and jump to it.
    fn ret(&mut self, enable_ir: bool) {
        self.pop(Reg16::PC);
        if enable_ir {
            self.ir_enabled = true;
        }
    }

    // Copy from given register into the memory address pointed to by given Reg16
    fn set_reg_ptr(&mut self, dst: Reg16, src: Reg8) {
        let addr = self.regs.get(src);
        let val = self.regs.get(dst);
        self.mem_set(addr, val);
    }

    // Copy value from (HL) into given register.
    fn get_reg_ptr(&mut self, dst: Reg8, src: Reg16) {
        let val = self.mem_get(self.regs.get(src));
        self.regs.set(dst, val);
    }

    // Copy value between A and (HL), then add or subtract HL.
    fn ldd_special(&mut self, is_get: bool, is_add: bool) {
        if is_get {
            self.get_reg_ptr(Reg8::A, Reg16::HL); // LD A, (HL+/-)
        } else {
            self.set_reg_ptr(Reg16::HL, Reg8::A); // LD (HL+/-), A
        }

        if is_add {
            self.regs.add(Reg16::HL, 1);
        } else {
            self.regs.sub(Reg16::HL, 1);
        }
    }

    // Load fast-page (0xFF00+) value to reg, or push reg value to a fast-page.
    fn ld_fast_page(&mut self, is_get: bool) {
        let addr = 0xff00 + self.regs.get(Reg8::C) as u16;
        if is_get {
            let val = self.mem_get(addr);
            self.regs.set(Reg8::A, val);
        } else {
            let val = self.regs.get(Reg8::A);
            self.mem_set(val, addr);
        }
    }

    // Write the stack pointer address to memory (two bytes).
    fn write_sp_to_ptr(&mut self, addr: u16) {
        let split_addr = util::split_u16(self.regs.get(Reg16::SP));
        self.mem_set(split_addr.0, addr);
        self.mem_set(split_addr.1, addr+1);
    }

    // Increment/decrement for (HL) value. TODO: should this be done another way? Maybe implement
    // it as a Reg8::HL_PTR, or something special in ALU?
    fn hl_ptr_inc_dec(&mut self, is_add: bool) {
        let addr = self.regs.get(Reg16::HL);
        let val = self.mem_get(addr);
        let val = if is_add { val + 1 } else { val - 1};
        self.mem_set(val, addr);
    }

    // Jump to the given address if Z or CY match what we expect
    fn jump_flag(&mut self, flag: Flag, if_unset: bool, addr: u16) {
        let flag_val = match flag {
            Flag::Z | Flag::CY => self.regs.get_flag(flag),
            _ => panic!("Can only call jump_flag on Z and CY flags.")
        };

        if flag_val ^ if_unset {
            self.regs.set(Reg16::PC, addr);
        }
    }

    fn jump_hl_ptr(&mut self) {
        let addr = self.regs.get(Reg16::HL);
        let addr = self.parse_u16(addr);
        self.regs.set(Reg16::PC, addr);
    }

    // Jump only if flag is set (or unset)
    fn jump_relative_flag(&mut self, flag: Flag, if_unset: bool, offset: u8) {
        let flag_val = match flag {
            Flag::Z | Flag::CY => self.regs.get_flag(flag),
            _ => panic!("Can only call jump_relative_flag on Z and CY flags.")
        };

        if flag_val ^ if_unset {
            self.jump_relative(offset);
        }
    }

    // Jump relative to current PC, where offset is twos-complement 8-bit signed int.
    fn jump_relative(&mut self, offset: u8) {
        let addr = self.regs.get(Reg16::PC) as i32;
        let addr = addr + (offset as i8) as i32;
        if addr < 0 || addr > 0xffff {
            println!("Fatal error: jumped out-of-bounds!");
            self.quit = true;
            return;
        }

        self.regs.set(Reg16::PC, addr as u16);
    }

    // Perform given ALU instruction against the given operands. It's the responsibility of other
    // functions to handle moving the result to a certain register, or setting necessary flags.
    fn alu(&mut self, op: AluOp, operand_a: u8, operand_b: u8) -> u8 {
        let carry_bit = self.regs.get_flag(Flag::CY);

        let op_a = operand_a;
        let op_b = operand_b;

        let result = match op {
            AluOp::Add(carry_op) => {
                let (val, overflow) = op_a.overflowing_add(op_b);
                let half_val = (op_a & 0xf) + (op_b & 0xf);
                self.half_carry = half_val > 0xf;
                self.full_carry = overflow;
                if carry_op && carry_bit {
                    let (new_val, new_overflow) = op_a.overflowing_add(1);
                    self.half_carry = self.half_carry || (half_val+1) > 0xf;
                    self.full_carry = self.full_carry || new_overflow;
                    new_val
                } else {
                    val // test
                }
            },
            AluOp::Sub(carry_op) => {
                let (val, overflow) = op_a.overflowing_sub(op_b);
                let (half_val, half_overflow) = (op_a & 0xf0).overflowing_sub(op_b & 0xf0);
                self.half_carry = half_overflow;
                self.full_carry = overflow;
                if carry_op && carry_bit {
                    let (new_val, new_overflow) = op_a.overflowing_sub(1);
                    self.half_carry = self.half_carry || (half_val-1) <= 0xf;
                    self.full_carry = self.full_carry || new_overflow;
                    new_val
                } else {
                    val
                }
            },
            AluOp::And      => op_a & op_b,
            AluOp::Xor      => op_a ^ op_b,
            AluOp::Or       => op_a | op_b,
            AluOp::Comp     => {
                self.half_carry = op_a < op_b;
                self.full_carry = (op_a & 0xf) < (op_b & 0xf);
                op_a
            },
            AluOp::RotateLeft(carry_op) => {
                let edge_bit = (op_a & 0x80) != 0;
                let rotate_bit = if carry_op { edge_bit } else { self.full_carry };
                self.full_carry = edge_bit;
                if rotate_bit {
                    (op_a << 1) | 0x1
                } else {
                    op_a << 1
                }
            },
            AluOp::RotateRight(carry_op) => {
                let edge_bit = (op_a & 0x1) != 0;
                let rotate_bit = if carry_op { edge_bit } else { self.full_carry };
                self.full_carry = edge_bit;
                if rotate_bit {
                    (op_a >> 1) | 0x80
                } else {
                    op_a >> 1
                }
            },
            AluOp::ShiftRight(is_arith) => {
                self.full_carry = (op_a & 0x1) != 0;
                let fill_bit = is_arith && (op_a & 0x80 != 0);
                if fill_bit {
                    (op_a >> 1) | 0x80
                } else {
                    op_a >> 1
                }
            },
            AluOp::ShiftLeft => {
                self.full_carry = (op_a & 0x80) != 0;
                op_a << 1
            },
            AluOp::Swap => {
                op_a.wrapping_shl(8) | op_a.wrapping_shr(8)
            },
            AluOp::Test(off) => {
                self.was_zero = (op_a & (0x1 << off)) == 0;
                op_a
            },
            AluOp::Set(off, val) => {
                let mask = 0x1 << off;
                if val {
                    op_a | mask
                } else {
                    op_a & (!mask)
                }
            }
        };

        self.was_zero = match op {
            AluOp::Comp | AluOp::Test(_) => self.was_zero,
            _ => result == 0
        };

        // if cfg!(debug_assertions) {
        //     println!("Result of ALU instruction {} with input {}, {} => {}. Z: {}, H: {}, CY: {}",
        //              op, op_a, op_b, result, self.was_zero, self.half_carry, self.full_carry);
        // }

        result
    }

    // As it turns out, adding/subtracting is really the only 16-bit ALU operation
    fn add_u16(&mut self, operand_a: u16, operand_b: u16, subtract: bool) -> u16 {
        match subtract {
            false => {
                let add16res = operand_a.overflowing_add(operand_b);
                self.full_carry = add16res.1;
                self.half_carry = ((operand_a & 0xfff) + (operand_b & 0xfff)) > 0xfff;
                add16res.0
            },
            true  => operand_a.wrapping_sub(operand_b)
        }
    }

    // Perform ALU op on accumulator and input register, and handle flags.
    fn arith_op(&mut self, op: AluOp, flags: FlagStatus, src: Reg8) {
        let operand_b = self.regs.get(src);
        self.arith_imm(op, Reg8::A, flags, operand_b);
    }

    fn arith_hl_ptr(&mut self, op: AluOp, flags: FlagStatus) {
        let operand_b = self.regs.get(Reg16::HL);
        let operand_b = self.mem_get(operand_b);
        self.arith_imm(op, Reg8::A, flags, operand_b);
    }

    fn bitwise_hl_ptr(&mut self, op: AluOp, flags: FlagStatus) {
        let addr = self.regs.get(Reg16::HL);
        let operand_a = self.mem_get(addr);
        let result = self.alu(op, operand_a, 0);
        self.mem_set(result, addr);
        self.evaluate_flags(flags);
    }

    // Take an immediate u8 instead of a register.
    fn arith_imm(&mut self, op: AluOp, dst_reg: Reg8, flags: FlagStatus, val: u8) {
        let operand_a = self.regs.get(dst_reg);
        let result = self.alu(op, operand_a, val);
        self.regs.set(dst_reg, result);
        self.evaluate_flags(flags);
    }

    // Add a 16 bit register to HL
    fn add_hl(&mut self, flags: FlagStatus, src: Reg16) {
        let operand_a = self.regs.get(Reg16::HL);
        let operand_b = self.regs.get(src);
        let result = self.add_u16(operand_a, operand_b, false);
        self.regs.set(Reg16::HL, result);
        self.evaluate_flags(flags);
    }

    // Add SP and immediate signed, and store to given Reg16.
    fn add_sp_signed(&mut self, flags: FlagStatus, dest: Reg16, offset: i8) {
        let sub = offset < 0;
        let offset_u = (offset as u8) as u16;
        let sp_val = self.regs.get(Reg16::SP);
        let sp_val = self.add_u16(sp_val, offset_u, sub);
        self.regs.set(dest, sp_val);
        self.evaluate_flags(flags);
    }

    fn evaluate_flags(&mut self, flags: FlagStatus) {
        self.evaluate_flag(Flag::Z,  flags.z);
        self.evaluate_flag(Flag::N,  flags.n);
        self.evaluate_flag(Flag::H,  flags.h);
        self.evaluate_flag(Flag::CY, flags.cy);
    }

    fn evaluate_flag(&mut self, flag: Flag, modifier: FlagMod) {
        match modifier {
            FlagMod::Ignore => (),
            FlagMod::Set(val) => self.regs.set_flag(flag, val),
            FlagMod::Eval => {
                match flag {
                    Flag::Z =>  self.regs.set_flag(flag, self.was_zero),
                    Flag::H =>  self.regs.set_flag(flag, self.half_carry),
                    Flag::CY => self.regs.set_flag(flag, self.full_carry),
                    Flag::N => panic!("Invalid FlagMod value for N flag! Cannot evaluate N with FlagMod::Eval.")
                }
            }
        }
    }

    fn decimal_adjust(&mut self, flags: FlagStatus) {
        let lo = self.regs.get(Reg8::A);
        let hi = lo.wrapping_shl(4);
        let lo = lo & 0xF;
        let mut adjust = 0;
        if !self.regs.get_flag(Flag::N) {
            if self.regs.get_flag(Flag::CY) || hi > 0x9 || lo > 0x9 {
                adjust += 0x60;
            }
            if self.regs.get_flag(Flag::H) || lo > 0x9 {
                adjust += 0x6;
            }
        } else {
            if self.regs.get_flag(Flag::CY) {
                if self.regs.get_flag(Flag::H) {
                    adjust += 0x9a;
                } else {
                    adjust += 0xa0;
                }
            } else if self.regs.get_flag(Flag::H) {
                adjust += 0xfa;
            }
        }

        self.arith_imm(AluOp::Add(false), Reg8::A, flags, adjust);
    }

    // Toggle the CY flag, used for CCF instruction
    fn toggle_cy(&mut self) {
        let val = !self.regs.get_flag(Flag::CY);
        self.regs.set_flag(Flag::CY, val);
    }

    // For HALT, just exit the program for now. TODO: Add accurate HALT emulation here.
    fn halt(&mut self) {
        println!("Encountered HALT instruction, exiting!");
        self.quit = true;
    }

    fn stop(&mut self) {
        println!("Encountered STOP instruction, exiting!");
        self.quit = true;
    }

    // Run the LCD, then process the current instruction.
    // TODO: This should eventually be cycle-accurate
    pub fn tick(&mut self) -> bool {
        self.ppu.tick();

        if !self.ppu.is_alive() {
            println!("Closed PPU window!");
            false
        } else {
            self.process()
        }
    }

    // Run the instruction at the current PC, return true if successful.
    pub fn process(&mut self) -> bool {
        if self.quit { return false; }
        let old_pc = self.regs.get(Reg16::PC);
        let opcode = self.mem_get(old_pc);
        let _operand8  = self.mem_get(old_pc+1);
        let _operand16 = self.parse_u16(old_pc+1);

        // Adjust opcode if it's a 0xcb prefixed instruction
        let opcode = if opcode == 0xcb {
            let newop = ((0xcb as u16) << 8) | _operand8 as u16;
            let _operand8  = self.mem_get(old_pc+2);
            let _operand16 = self.parse_u16(old_pc+2);
            newop
        } else {
            opcode as u16
        };

        let inst = lookup::get_instruction(opcode);

        // Increment PC before we process the instruction. During execution the current PC will
        // represent the next instruction to process.
        let mut bytes = inst.bytes as u16;
        if inst.prefix_cb {
            bytes += 1; // TODO: Fix this in the lookup table
        }
        self.regs.set(Reg16::PC, old_pc + bytes);

        match opcode {
            // [0x00, 0x3f] - Load, INC/DEC, some jumps, and other various instructions.
            0x00 => (),
            0x01 => self.regs.set(Reg16::BC, _operand16),
            0x02 => self.set_reg_ptr(Reg16::BC, Reg8::A),
            0x03 => self.regs.add(Reg16::BC, 1),
            0x04 => self.regs.add(Reg8::B, 1),
            0x05 => self.regs.sub(Reg8::B, 1),
            0x06 => self.regs.set(Reg8::B, _operand8),
            0x07 => self.arith_imm(AluOp::RotateLeft(true), Reg8::A, lookup::get_flags(opcode), 0),
            0x08 => self.write_sp_to_ptr(_operand16),
            0x09 => self.add_hl(lookup::get_flags(opcode), Reg16::BC),
            0x0a => self.get_reg_ptr(Reg8::A, Reg16::BC),
            0x0b => self.regs.sub(Reg16::BC, 1),
            0x0c => self.regs.add(Reg8::C, 1),
            0x0d => self.regs.sub(Reg8::C, 1),
            0x0e => self.regs.set(Reg8::C, _operand8),
            0x0f => self.arith_imm(AluOp::RotateRight(true), Reg8::A, lookup::get_flags(opcode), 0),
            0x10 => self.stop(),
            0x11 => self.regs.set(Reg16::DE, _operand16),
            0x12 => self.set_reg_ptr(Reg16::DE, Reg8::A),
            0x13 => self.regs.add(Reg16::DE, 1),
            0x14 => self.regs.add(Reg8::D, 1),
            0x15 => self.regs.sub(Reg8::D, 1),
            0x16 => self.regs.set(Reg8::D, _operand8),
            0x17 => self.arith_imm(AluOp::RotateLeft(false), Reg8::A, lookup::get_flags(opcode), 0),
            0x18 => self.jump_relative(_operand8),
            0x19 => self.add_hl(lookup::get_flags(opcode), Reg16::DE),
            0x1a => self.get_reg_ptr(Reg8::A, Reg16::DE),
            0x1b => self.regs.sub(Reg16::BC, 1),
            0x1c => self.regs.add(Reg8::D, 1),
            0x1d => self.regs.sub(Reg8::D, 1),
            0x1e => self.regs.set(Reg8::E, _operand8),
            0x1f => self.arith_imm(AluOp::RotateRight(false), Reg8::A, lookup::get_flags(opcode), 0),
            0x20 => self.jump_relative_flag(Flag::Z, true, _operand8),
            0x21 => self.regs.set(Reg16::HL, _operand16),
            0x22 => self.ldd_special(true, true),
            0x23 => self.regs.add(Reg16::HL, 1),
            0x24 => self.regs.add(Reg8::H, 1),
            0x25 => self.regs.sub(Reg8::H, 1),
            0x26 => self.regs.set(Reg8::H, _operand8),
            0x27 => self.decimal_adjust(lookup::get_flags(opcode)),
            0x28 => self.jump_relative_flag(Flag::Z, false, _operand8),
            0x29 => self.add_hl(lookup::get_flags(opcode), Reg16::HL),
            0x2a => self.ldd_special(false, true),
            0x2b => self.regs.sub(Reg16::HL, 1),
            0x2c => self.regs.add(Reg8::L, 1),
            0x2d => self.regs.sub(Reg8::L, 1),
            0x2e => self.regs.set(Reg8::L, _operand8),
            0x2f => self.arith_imm(AluOp::Xor, Reg8::A, lookup::get_flags(opcode), 0xff),
            0x30 => self.jump_relative_flag(Flag::CY, true, _operand8),
            0x31 => self.regs.set(Reg16::SP, _operand16),
            0x32 => self.ldd_special(true, false),
            0x33 => self.regs.add(Reg16::HL, 1),
            0x34 => self.hl_ptr_inc_dec(true),
            0x35 => self.hl_ptr_inc_dec(false),
            0x36 => {let hl = self.regs.get(Reg16::HL); self.mem_set(_operand8, hl)},
            0x37 => self.regs.set_flag(Flag::CY, true),
            0x38 => self.jump_relative_flag(Flag::CY, false, _operand8),
            0x39 => self.add_hl(lookup::get_flags(opcode), Reg16::SP),
            0x3a => self.ldd_special(false, false),
            0x3b => self.regs.sub(Reg16::SP, 1),
            0x3c => self.regs.add(Reg8::A, 1),
            0x3d => self.regs.sub(Reg8::A, 1),
            0x3e => self.regs.set(Reg8::A, _operand8),
            0x3f => self.toggle_cy(),

            // [0x40, 0x7f] - Mostly copy instructions between registers and (HL).
            0x40 => self.regs.copy(Reg8::B, Reg8::B),
            0x41 => self.regs.copy(Reg8::B, Reg8::C),
            0x42 => self.regs.copy(Reg8::B, Reg8::D),
            0x43 => self.regs.copy(Reg8::B, Reg8::E),
            0x44 => self.regs.copy(Reg8::B, Reg8::H),
            0x45 => self.regs.copy(Reg8::B, Reg8::L),
            0x46 => self.get_reg_ptr(Reg8::B, Reg16::HL),
            0x47 => self.regs.copy(Reg8::B, Reg8::A),
            0x48 => self.regs.copy(Reg8::C, Reg8::B),
            0x49 => self.regs.copy(Reg8::C, Reg8::C),
            0x4a => self.regs.copy(Reg8::C, Reg8::D),
            0x4b => self.regs.copy(Reg8::C, Reg8::E),
            0x4c => self.regs.copy(Reg8::C, Reg8::H),
            0x4d => self.regs.copy(Reg8::C, Reg8::L),
            0x4e => self.get_reg_ptr(Reg8::C, Reg16::HL),
            0x4f => self.regs.copy(Reg8::C, Reg8::A),
            0x50 => self.regs.copy(Reg8::D, Reg8::B),
            0x51 => self.regs.copy(Reg8::D, Reg8::C),
            0x52 => self.regs.copy(Reg8::D, Reg8::D),
            0x53 => self.regs.copy(Reg8::D, Reg8::E),
            0x54 => self.regs.copy(Reg8::D, Reg8::H),
            0x55 => self.regs.copy(Reg8::D, Reg8::L),
            0x56 => self.get_reg_ptr(Reg8::D, Reg16::HL),
            0x57 => self.regs.copy(Reg8::D, Reg8::A),
            0x58 => self.regs.copy(Reg8::E, Reg8::B),
            0x59 => self.regs.copy(Reg8::E, Reg8::C),
            0x5a => self.regs.copy(Reg8::E, Reg8::D),
            0x5b => self.regs.copy(Reg8::E, Reg8::E),
            0x5c => self.regs.copy(Reg8::E, Reg8::H),
            0x5d => self.regs.copy(Reg8::E, Reg8::L),
            0x5e => self.get_reg_ptr(Reg8::E, Reg16::HL),
            0x5f => self.regs.copy(Reg8::E, Reg8::A),
            0x60 => self.regs.copy(Reg8::H, Reg8::B),
            0x61 => self.regs.copy(Reg8::H, Reg8::C),
            0x62 => self.regs.copy(Reg8::H, Reg8::D),
            0x63 => self.regs.copy(Reg8::H, Reg8::E),
            0x64 => self.regs.copy(Reg8::H, Reg8::H),
            0x65 => self.regs.copy(Reg8::H, Reg8::L),
            0x66 => self.get_reg_ptr(Reg8::H, Reg16::HL),
            0x67 => self.regs.copy(Reg8::H, Reg8::A),
            0x68 => self.regs.copy(Reg8::L, Reg8::B),
            0x69 => self.regs.copy(Reg8::L, Reg8::C),
            0x6a => self.regs.copy(Reg8::L, Reg8::D),
            0x6b => self.regs.copy(Reg8::L, Reg8::E),
            0x6c => self.regs.copy(Reg8::L, Reg8::H),
            0x6d => self.regs.copy(Reg8::L, Reg8::L),
            0x6e => self.get_reg_ptr(Reg8::L, Reg16::HL),
            0x6f => self.regs.copy(Reg8::L, Reg8::A),
            0x70 => self.set_reg_ptr(Reg16::HL, Reg8::B),
            0x71 => self.set_reg_ptr(Reg16::HL, Reg8::C),
            0x72 => self.set_reg_ptr(Reg16::HL, Reg8::D),
            0x73 => self.set_reg_ptr(Reg16::HL, Reg8::E),
            0x74 => self.set_reg_ptr(Reg16::HL, Reg8::H),
            0x75 => self.set_reg_ptr(Reg16::HL, Reg8::L),
            0x76 => self.halt(),
            0x77 => self.set_reg_ptr(Reg16::HL, Reg8::A),
            0x78 => self.regs.copy(Reg8::A, Reg8::B),
            0x79 => self.regs.copy(Reg8::A, Reg8::C),
            0x7a => self.regs.copy(Reg8::A, Reg8::D),
            0x7b => self.regs.copy(Reg8::A, Reg8::E),
            0x7c => self.regs.copy(Reg8::A, Reg8::H),
            0x7d => self.regs.copy(Reg8::A, Reg8::L),
            0x7e => self.get_reg_ptr(Reg8::A, Reg16::HL),
            0x7f => self.regs.copy(Reg8::A, Reg8::A),

            // [0x80, 0xbf] - Arithmetic operations
            0x80 => self.arith_op(AluOp::Add(false), lookup::get_flags(opcode), Reg8::B),
            0x81 => self.arith_op(AluOp::Add(false), lookup::get_flags(opcode), Reg8::C),
            0x82 => self.arith_op(AluOp::Add(false), lookup::get_flags(opcode), Reg8::D),
            0x83 => self.arith_op(AluOp::Add(false), lookup::get_flags(opcode), Reg8::E),
            0x84 => self.arith_op(AluOp::Add(false), lookup::get_flags(opcode), Reg8::H),
            0x85 => self.arith_op(AluOp::Add(false), lookup::get_flags(opcode), Reg8::L),
            0x86 => self.arith_hl_ptr(AluOp::Add(false), lookup::get_flags(opcode)),
            0x87 => self.arith_op(AluOp::Add(false), lookup::get_flags(opcode), Reg8::A),
            0x88 => self.arith_op(AluOp::Add(true), lookup::get_flags(opcode), Reg8::B),
            0x89 => self.arith_op(AluOp::Add(true), lookup::get_flags(opcode), Reg8::C),
            0x8a => self.arith_op(AluOp::Add(true), lookup::get_flags(opcode), Reg8::D),
            0x8b => self.arith_op(AluOp::Add(true), lookup::get_flags(opcode), Reg8::E),
            0x8c => self.arith_op(AluOp::Add(true), lookup::get_flags(opcode), Reg8::H),
            0x8d => self.arith_op(AluOp::Add(true), lookup::get_flags(opcode), Reg8::L),
            0x8e => self.arith_hl_ptr(AluOp::Add(true), lookup::get_flags(opcode)),
            0x8f => self.arith_op(AluOp::Add(true), lookup::get_flags(opcode), Reg8::A),
            0x90 => self.arith_op(AluOp::Sub(false), lookup::get_flags(opcode), Reg8::B),
            0x91 => self.arith_op(AluOp::Sub(false), lookup::get_flags(opcode), Reg8::C),
            0x92 => self.arith_op(AluOp::Sub(false), lookup::get_flags(opcode), Reg8::D),
            0x93 => self.arith_op(AluOp::Sub(false), lookup::get_flags(opcode), Reg8::E),
            0x94 => self.arith_op(AluOp::Sub(false), lookup::get_flags(opcode), Reg8::H),
            0x95 => self.arith_op(AluOp::Sub(false), lookup::get_flags(opcode), Reg8::L),
            0x96 => self.arith_hl_ptr(AluOp::Sub(false), lookup::get_flags(opcode)),
            0x97 => self.arith_op(AluOp::Sub(false), lookup::get_flags(opcode), Reg8::A),
            0x98 => self.arith_op(AluOp::Sub(true), lookup::get_flags(opcode), Reg8::B),
            0x99 => self.arith_op(AluOp::Sub(true), lookup::get_flags(opcode), Reg8::C),
            0x9a => self.arith_op(AluOp::Sub(true), lookup::get_flags(opcode), Reg8::D),
            0x9b => self.arith_op(AluOp::Sub(true), lookup::get_flags(opcode), Reg8::E),
            0x9c => self.arith_op(AluOp::Sub(true), lookup::get_flags(opcode), Reg8::H),
            0x9d => self.arith_op(AluOp::Sub(true), lookup::get_flags(opcode), Reg8::L),
            0x9e => self.arith_hl_ptr(AluOp::Sub(true), lookup::get_flags(opcode)),
            0x9f => self.arith_op(AluOp::Sub(true), lookup::get_flags(opcode), Reg8::A),
            0xa0 => self.arith_op(AluOp::And, lookup::get_flags(opcode), Reg8::B),
            0xa1 => self.arith_op(AluOp::And, lookup::get_flags(opcode), Reg8::C),
            0xa2 => self.arith_op(AluOp::And, lookup::get_flags(opcode), Reg8::D),
            0xa3 => self.arith_op(AluOp::And, lookup::get_flags(opcode), Reg8::E),
            0xa4 => self.arith_op(AluOp::And, lookup::get_flags(opcode), Reg8::H),
            0xa5 => self.arith_op(AluOp::And, lookup::get_flags(opcode), Reg8::L),
            0xa6 => self.arith_hl_ptr(AluOp::And, lookup::get_flags(opcode)),
            0xa7 => self.arith_op(AluOp::And, lookup::get_flags(opcode), Reg8::A),
            0xa8 => self.arith_op(AluOp::Xor, lookup::get_flags(opcode), Reg8::B),
            0xa9 => self.arith_op(AluOp::Xor, lookup::get_flags(opcode), Reg8::C),
            0xaa => self.arith_op(AluOp::Xor, lookup::get_flags(opcode), Reg8::D),
            0xab => self.arith_op(AluOp::Xor, lookup::get_flags(opcode), Reg8::E),
            0xac => self.arith_op(AluOp::Xor, lookup::get_flags(opcode), Reg8::H),
            0xad => self.arith_op(AluOp::Xor, lookup::get_flags(opcode), Reg8::L),
            0xae => self.arith_hl_ptr(AluOp::Xor, lookup::get_flags(opcode)),
            0xaf => self.arith_op(AluOp::Xor, lookup::get_flags(opcode), Reg8::A),
            0xb0 => self.arith_op(AluOp::Or, lookup::get_flags(opcode), Reg8::B),
            0xb1 => self.arith_op(AluOp::Or, lookup::get_flags(opcode), Reg8::C),
            0xb2 => self.arith_op(AluOp::Or, lookup::get_flags(opcode), Reg8::D),
            0xb3 => self.arith_op(AluOp::Or, lookup::get_flags(opcode), Reg8::E),
            0xb4 => self.arith_op(AluOp::Or, lookup::get_flags(opcode), Reg8::H),
            0xb5 => self.arith_op(AluOp::Or, lookup::get_flags(opcode), Reg8::L),
            0xb6 => self.arith_hl_ptr(AluOp::Or, lookup::get_flags(opcode)),
            0xb7 => self.arith_op(AluOp::Or, lookup::get_flags(opcode), Reg8::A),
            0xb8 => self.arith_op(AluOp::Comp, lookup::get_flags(opcode), Reg8::B),
            0xb9 => self.arith_op(AluOp::Comp, lookup::get_flags(opcode), Reg8::C),
            0xba => self.arith_op(AluOp::Comp, lookup::get_flags(opcode), Reg8::D),
            0xbb => self.arith_op(AluOp::Comp, lookup::get_flags(opcode), Reg8::E),
            0xbc => self.arith_op(AluOp::Comp, lookup::get_flags(opcode), Reg8::H),
            0xbd => self.arith_op(AluOp::Comp, lookup::get_flags(opcode), Reg8::L),
            0xbe => self.arith_hl_ptr(AluOp::Comp, lookup::get_flags(opcode)),
            0xbf => self.arith_op(AluOp::Comp, lookup::get_flags(opcode), Reg8::A),

            // [0xc0, 0xff] - Flow control, push/pop/call/ret, and other various instructions.
            0xc0 => self.ret_flag(Flag::Z, true),
            0xc1 => self.pop(Reg16::BC),
            0xc2 => self.jump_flag(Flag::Z, true, _operand16),
            0xc3 => self.regs.set(Reg16::PC, _operand16),
            0xc4 => self.call_flag(Flag::Z, true, _operand16),
            0xc5 => self.push(Reg16::BC),
            0xc6 => self.arith_imm(AluOp::Add(false), Reg8::A, lookup::get_flags(opcode), _operand8),
            0xc7 => self.call(0x00),
            0xc8 => self.ret_flag(Flag::Z, false),
            0xc9 => self.ret(false),
            0xca => self.jump_flag(Flag::Z, false, _operand16),
            0xcb => self.quit = true, // This shouldn't ever happen
            0xcc => self.call_flag(Flag::Z, false, _operand16),
            0xcd => self.call(_operand16),
            0xce => self.arith_imm(AluOp::Add(true), Reg8::A, lookup::get_flags(opcode), _operand8),
            0xcf => self.call(0x08),
            0xd0 => self.ret_flag(Flag::CY, true),
            0xd1 => self.pop(Reg16::DE),
            0xd2 => self.jump_flag(Flag::CY, true, _operand16),
            0xd3 => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xd4 => self.call_flag(Flag::CY, true, _operand16),
            0xd5 => self.push(Reg16::DE),
            0xd6 => self.arith_imm(AluOp::Sub(false), Reg8::A, lookup::get_flags(opcode), _operand8),
            0xd7 => self.call(0x10),
            0xd8 => self.ret_flag(Flag::CY, false),
            0xd9 => self.ret(true),
            0xda => self.jump_flag(Flag::CY, false, _operand16),
            0xdb => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xdc => self.call_flag(Flag::CY, false, _operand16),
            0xdd => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xde => self.arith_imm(AluOp::Sub(true), Reg8::A, lookup::get_flags(opcode), _operand8),
            0xdf => self.call(0x18),
            0xe0 => {let a = self.regs.get(Reg8::A); self.mem_set(a, 0xff00 + (_operand8 as u16))},
            0xe1 => self.pop(Reg16::HL),
            0xe2 => self.ld_fast_page(true),
            0xe3 => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xe4 => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xe5 => self.push(Reg16::HL),
            0xe6 => self.arith_imm(AluOp::And, Reg8::A, lookup::get_flags(opcode), _operand8),
            0xe7 => self.call(0x20),
            0xe8 => self.add_sp_signed(lookup::get_flags(opcode), Reg16::SP, _operand8 as i8),
            0xe9 => self.jump_hl_ptr(),
            0xea => {let a = self.regs.get(Reg8::A); self.mem_set(a, _operand16)},
            0xeb => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xec => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xed => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xee => self.arith_imm(AluOp::Xor, Reg8::A, lookup::get_flags(opcode), _operand8),
            0xef => self.call(0x28),
            0xf0 => {let val = self.mem_get(0xff00 + (_operand8 as u16)); self.regs.set(Reg8::A, val)},
            0xf1 => self.pop(Reg16::AF),
            0xf2 => self.ld_fast_page(false),
            0xf3 => self.ir_enabled = false,
            0xf4 => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xf5 => self.push(Reg16::AF),
            0xf6 => self.arith_imm(AluOp::Or, Reg8::A, lookup::get_flags(opcode), _operand8),
            0xf7 => self.call(0x30),
            0xf8 => self.add_sp_signed(lookup::get_flags(opcode), Reg16::HL, _operand8 as i8),
            0xf9 => self.regs.copy(Reg16::SP, Reg16::HL),
            0xfa => {let val = self.mem_get(_operand16); self.regs.set(Reg8::A, val)},
            0xfb => self.ir_enabled = true,
            0xfc => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xfd => panic!("Received invalid instruction UNKNOWN_{:02X}", opcode),
            0xfe => self.arith_imm(AluOp::Comp, Reg8::A, lookup::get_flags(opcode), _operand8),
            0xff => self.call(0x38),

            // [0xcb00, 0xcb3f] - Bitwise rotate, shift, and swap.
            0xcb00 => self.arith_imm(AluOp::RotateLeft(true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb01 => self.arith_imm(AluOp::RotateLeft(true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb02 => self.arith_imm(AluOp::RotateLeft(true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb03 => self.arith_imm(AluOp::RotateLeft(true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb04 => self.arith_imm(AluOp::RotateLeft(true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb05 => self.arith_imm(AluOp::RotateLeft(true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb06 => self.bitwise_hl_ptr(AluOp::RotateLeft(true), lookup::get_flags(opcode)),
            0xcb07 => self.arith_imm(AluOp::RotateLeft(true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb08 => self.arith_imm(AluOp::RotateRight(true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb09 => self.arith_imm(AluOp::RotateRight(true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb0a => self.arith_imm(AluOp::RotateRight(true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb0b => self.arith_imm(AluOp::RotateRight(true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb0c => self.arith_imm(AluOp::RotateRight(true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb0d => self.arith_imm(AluOp::RotateRight(true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb0e => self.bitwise_hl_ptr(AluOp::RotateRight(true), lookup::get_flags(opcode)),
            0xcb0f => self.arith_imm(AluOp::RotateRight(true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb10 => self.arith_imm(AluOp::RotateLeft(false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb11 => self.arith_imm(AluOp::RotateLeft(false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb12 => self.arith_imm(AluOp::RotateLeft(false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb13 => self.arith_imm(AluOp::RotateLeft(false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb14 => self.arith_imm(AluOp::RotateLeft(false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb15 => self.arith_imm(AluOp::RotateLeft(false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb16 => self.bitwise_hl_ptr(AluOp::RotateLeft(false), lookup::get_flags(opcode)),
            0xcb17 => self.arith_imm(AluOp::RotateLeft(false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb18 => self.arith_imm(AluOp::RotateRight(false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb19 => self.arith_imm(AluOp::RotateRight(false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb1a => self.arith_imm(AluOp::RotateRight(false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb1b => self.arith_imm(AluOp::RotateRight(false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb1c => self.arith_imm(AluOp::RotateRight(false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb1d => self.arith_imm(AluOp::RotateRight(false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb1e => self.bitwise_hl_ptr(AluOp::RotateRight(false), lookup::get_flags(opcode)),
            0xcb1f => self.arith_imm(AluOp::RotateRight(false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb20 => self.arith_imm(AluOp::ShiftLeft, Reg8::B, lookup::get_flags(opcode), 0),
            0xcb21 => self.arith_imm(AluOp::ShiftLeft, Reg8::C, lookup::get_flags(opcode), 0),
            0xcb22 => self.arith_imm(AluOp::ShiftLeft, Reg8::D, lookup::get_flags(opcode), 0),
            0xcb23 => self.arith_imm(AluOp::ShiftLeft, Reg8::E, lookup::get_flags(opcode), 0),
            0xcb24 => self.arith_imm(AluOp::ShiftLeft, Reg8::H, lookup::get_flags(opcode), 0),
            0xcb25 => self.arith_imm(AluOp::ShiftLeft, Reg8::L, lookup::get_flags(opcode), 0),
            0xcb26 => self.bitwise_hl_ptr(AluOp::ShiftLeft, lookup::get_flags(opcode)),
            0xcb27 => self.arith_imm(AluOp::ShiftLeft, Reg8::A, lookup::get_flags(opcode), 0),
            0xcb28 => self.arith_imm(AluOp::ShiftRight(true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb29 => self.arith_imm(AluOp::ShiftRight(true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb2a => self.arith_imm(AluOp::ShiftRight(true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb2b => self.arith_imm(AluOp::ShiftRight(true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb2c => self.arith_imm(AluOp::ShiftRight(true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb2d => self.arith_imm(AluOp::ShiftRight(true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb2e => self.bitwise_hl_ptr(AluOp::ShiftRight(true), lookup::get_flags(opcode)),
            0xcb2f => self.arith_imm(AluOp::ShiftRight(true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb30 => self.arith_imm(AluOp::Swap, Reg8::B, lookup::get_flags(opcode), 0),
            0xcb31 => self.arith_imm(AluOp::Swap, Reg8::C, lookup::get_flags(opcode), 0),
            0xcb32 => self.arith_imm(AluOp::Swap, Reg8::D, lookup::get_flags(opcode), 0),
            0xcb33 => self.arith_imm(AluOp::Swap, Reg8::E, lookup::get_flags(opcode), 0),
            0xcb34 => self.arith_imm(AluOp::Swap, Reg8::H, lookup::get_flags(opcode), 0),
            0xcb35 => self.arith_imm(AluOp::Swap, Reg8::L, lookup::get_flags(opcode), 0),
            0xcb36 => self.bitwise_hl_ptr(AluOp::Swap, lookup::get_flags(opcode)),
            0xcb37 => self.arith_imm(AluOp::Swap, Reg8::A, lookup::get_flags(opcode), 0),
            0xcb38 => self.arith_imm(AluOp::ShiftRight(false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb39 => self.arith_imm(AluOp::ShiftRight(false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb3a => self.arith_imm(AluOp::ShiftRight(false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb3b => self.arith_imm(AluOp::ShiftRight(false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb3c => self.arith_imm(AluOp::ShiftRight(false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb3d => self.arith_imm(AluOp::ShiftRight(false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb3e => self.bitwise_hl_ptr(AluOp::ShiftRight(false), lookup::get_flags(opcode)),
            0xcb3f => self.arith_imm(AluOp::ShiftRight(false), Reg8::A, lookup::get_flags(opcode), 0),

            // [0xcb40, 0xcb7f] - Bit test, push value to Z flag
            0xcb40 => self.arith_imm(AluOp::Test(0), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb41 => self.arith_imm(AluOp::Test(0), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb42 => self.arith_imm(AluOp::Test(0), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb43 => self.arith_imm(AluOp::Test(0), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb44 => self.arith_imm(AluOp::Test(0), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb45 => self.arith_imm(AluOp::Test(0), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb46 => self.bitwise_hl_ptr(AluOp::Test(0), lookup::get_flags(opcode)),
            0xcb47 => self.arith_imm(AluOp::Test(0), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb48 => self.arith_imm(AluOp::Test(1), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb49 => self.arith_imm(AluOp::Test(1), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb4a => self.arith_imm(AluOp::Test(1), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb4b => self.arith_imm(AluOp::Test(1), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb4c => self.arith_imm(AluOp::Test(1), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb4d => self.arith_imm(AluOp::Test(1), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb4e => self.bitwise_hl_ptr(AluOp::Test(1), lookup::get_flags(opcode)),
            0xcb4f => self.arith_imm(AluOp::Test(1), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb50 => self.arith_imm(AluOp::Test(2), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb51 => self.arith_imm(AluOp::Test(2), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb52 => self.arith_imm(AluOp::Test(2), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb53 => self.arith_imm(AluOp::Test(2), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb54 => self.arith_imm(AluOp::Test(2), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb55 => self.arith_imm(AluOp::Test(2), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb56 => self.bitwise_hl_ptr(AluOp::Test(2), lookup::get_flags(opcode)),
            0xcb57 => self.arith_imm(AluOp::Test(2), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb58 => self.arith_imm(AluOp::Test(3), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb59 => self.arith_imm(AluOp::Test(3), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb5a => self.arith_imm(AluOp::Test(3), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb5b => self.arith_imm(AluOp::Test(3), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb5c => self.arith_imm(AluOp::Test(3), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb5d => self.arith_imm(AluOp::Test(3), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb5e => self.bitwise_hl_ptr(AluOp::Test(3), lookup::get_flags(opcode)),
            0xcb5f => self.arith_imm(AluOp::Test(3), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb60 => self.arith_imm(AluOp::Test(4), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb61 => self.arith_imm(AluOp::Test(4), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb62 => self.arith_imm(AluOp::Test(4), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb63 => self.arith_imm(AluOp::Test(4), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb64 => self.arith_imm(AluOp::Test(4), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb65 => self.arith_imm(AluOp::Test(4), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb66 => self.bitwise_hl_ptr(AluOp::Test(4), lookup::get_flags(opcode)),
            0xcb67 => self.arith_imm(AluOp::Test(4), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb68 => self.arith_imm(AluOp::Test(5), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb69 => self.arith_imm(AluOp::Test(5), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb6a => self.arith_imm(AluOp::Test(5), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb6b => self.arith_imm(AluOp::Test(5), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb6c => self.arith_imm(AluOp::Test(5), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb6d => self.arith_imm(AluOp::Test(5), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb6e => self.bitwise_hl_ptr(AluOp::Test(5), lookup::get_flags(opcode)),
            0xcb6f => self.arith_imm(AluOp::Test(5), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb70 => self.arith_imm(AluOp::Test(6), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb71 => self.arith_imm(AluOp::Test(6), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb72 => self.arith_imm(AluOp::Test(6), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb73 => self.arith_imm(AluOp::Test(6), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb74 => self.arith_imm(AluOp::Test(6), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb75 => self.arith_imm(AluOp::Test(6), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb76 => self.bitwise_hl_ptr(AluOp::Test(6), lookup::get_flags(opcode)),
            0xcb77 => self.arith_imm(AluOp::Test(6), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb78 => self.arith_imm(AluOp::Test(7), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb79 => self.arith_imm(AluOp::Test(7), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb7a => self.arith_imm(AluOp::Test(7), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb7b => self.arith_imm(AluOp::Test(7), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb7c => self.arith_imm(AluOp::Test(7), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb7d => self.arith_imm(AluOp::Test(7), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb7e => self.bitwise_hl_ptr(AluOp::Test(7), lookup::get_flags(opcode)),
            0xcb7f => self.arith_imm(AluOp::Test(7), Reg8::A, lookup::get_flags(opcode), 0),

            // [0xcb80, 0xcbb9] - Reset bit to 0
            0xcb80 => self.arith_imm(AluOp::Set(0, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb81 => self.arith_imm(AluOp::Set(0, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb82 => self.arith_imm(AluOp::Set(0, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb83 => self.arith_imm(AluOp::Set(0, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb84 => self.arith_imm(AluOp::Set(0, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb85 => self.arith_imm(AluOp::Set(0, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb86 => self.bitwise_hl_ptr(AluOp::Set(0, false), lookup::get_flags(opcode)),
            0xcb87 => self.arith_imm(AluOp::Set(0, false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb88 => self.arith_imm(AluOp::Set(1, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb89 => self.arith_imm(AluOp::Set(1, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb8a => self.arith_imm(AluOp::Set(1, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb8b => self.arith_imm(AluOp::Set(1, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb8c => self.arith_imm(AluOp::Set(1, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb8d => self.arith_imm(AluOp::Set(1, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb8e => self.bitwise_hl_ptr(AluOp::Set(1, false), lookup::get_flags(opcode)),
            0xcb8f => self.arith_imm(AluOp::Set(1, false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb90 => self.arith_imm(AluOp::Set(2, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb91 => self.arith_imm(AluOp::Set(2, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb92 => self.arith_imm(AluOp::Set(2, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb93 => self.arith_imm(AluOp::Set(2, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb94 => self.arith_imm(AluOp::Set(2, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb95 => self.arith_imm(AluOp::Set(2, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb96 => self.bitwise_hl_ptr(AluOp::Set(2, false), lookup::get_flags(opcode)),
            0xcb97 => self.arith_imm(AluOp::Set(2, false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcb98 => self.arith_imm(AluOp::Set(3, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcb99 => self.arith_imm(AluOp::Set(3, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcb9a => self.arith_imm(AluOp::Set(3, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcb9b => self.arith_imm(AluOp::Set(3, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcb9c => self.arith_imm(AluOp::Set(3, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcb9d => self.arith_imm(AluOp::Set(3, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcb9e => self.bitwise_hl_ptr(AluOp::Set(3, false), lookup::get_flags(opcode)),
            0xcb9f => self.arith_imm(AluOp::Set(3, false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcba0 => self.arith_imm(AluOp::Set(4, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcba1 => self.arith_imm(AluOp::Set(4, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcba2 => self.arith_imm(AluOp::Set(4, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcba3 => self.arith_imm(AluOp::Set(4, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcba4 => self.arith_imm(AluOp::Set(4, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcba5 => self.arith_imm(AluOp::Set(4, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcba6 => self.bitwise_hl_ptr(AluOp::Set(4, false), lookup::get_flags(opcode)),
            0xcba7 => self.arith_imm(AluOp::Set(4, false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcba8 => self.arith_imm(AluOp::Set(5, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcba9 => self.arith_imm(AluOp::Set(5, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbaa => self.arith_imm(AluOp::Set(5, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbab => self.arith_imm(AluOp::Set(5, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbac => self.arith_imm(AluOp::Set(5, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbad => self.arith_imm(AluOp::Set(5, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbae => self.bitwise_hl_ptr(AluOp::Set(5, false), lookup::get_flags(opcode)),
            0xcbaf => self.arith_imm(AluOp::Set(5, false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbb0 => self.arith_imm(AluOp::Set(6, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbb1 => self.arith_imm(AluOp::Set(6, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbb2 => self.arith_imm(AluOp::Set(6, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbb3 => self.arith_imm(AluOp::Set(6, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbb4 => self.arith_imm(AluOp::Set(6, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbb5 => self.arith_imm(AluOp::Set(6, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbb6 => self.bitwise_hl_ptr(AluOp::Set(6, false), lookup::get_flags(opcode)),
            0xcbb7 => self.arith_imm(AluOp::Set(6, false), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbb8 => self.arith_imm(AluOp::Set(7, false), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbb9 => self.arith_imm(AluOp::Set(7, false), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbba => self.arith_imm(AluOp::Set(7, false), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbbb => self.arith_imm(AluOp::Set(7, false), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbbc => self.arith_imm(AluOp::Set(7, false), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbbd => self.arith_imm(AluOp::Set(7, false), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbbe => self.bitwise_hl_ptr(AluOp::Set(7, false), lookup::get_flags(opcode)),
            0xcbbf => self.arith_imm(AluOp::Set(7, false), Reg8::A, lookup::get_flags(opcode), 0),

            // [0xcbc0, 0xcbf9] - Set bit to 1
            0xcbc0 => self.arith_imm(AluOp::Set(0, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbc1 => self.arith_imm(AluOp::Set(0, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbc2 => self.arith_imm(AluOp::Set(0, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbc3 => self.arith_imm(AluOp::Set(0, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbc4 => self.arith_imm(AluOp::Set(0, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbc5 => self.arith_imm(AluOp::Set(0, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbc6 => self.bitwise_hl_ptr(AluOp::Set(0, true), lookup::get_flags(opcode)),
            0xcbc7 => self.arith_imm(AluOp::Set(0, true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbc8 => self.arith_imm(AluOp::Set(1, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbc9 => self.arith_imm(AluOp::Set(1, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbca => self.arith_imm(AluOp::Set(1, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbcb => self.arith_imm(AluOp::Set(1, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbcc => self.arith_imm(AluOp::Set(1, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbcd => self.arith_imm(AluOp::Set(1, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbce => self.bitwise_hl_ptr(AluOp::Set(1, true), lookup::get_flags(opcode)),
            0xcbcf => self.arith_imm(AluOp::Set(1, true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbd0 => self.arith_imm(AluOp::Set(2, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbd1 => self.arith_imm(AluOp::Set(2, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbd2 => self.arith_imm(AluOp::Set(2, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbd3 => self.arith_imm(AluOp::Set(2, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbd4 => self.arith_imm(AluOp::Set(2, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbd5 => self.arith_imm(AluOp::Set(2, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbd6 => self.bitwise_hl_ptr(AluOp::Set(2, true), lookup::get_flags(opcode)),
            0xcbd7 => self.arith_imm(AluOp::Set(2, true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbd8 => self.arith_imm(AluOp::Set(3, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbd9 => self.arith_imm(AluOp::Set(3, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbda => self.arith_imm(AluOp::Set(3, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbdb => self.arith_imm(AluOp::Set(3, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbdc => self.arith_imm(AluOp::Set(3, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbdd => self.arith_imm(AluOp::Set(3, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbde => self.bitwise_hl_ptr(AluOp::Set(3, true), lookup::get_flags(opcode)),
            0xcbdf => self.arith_imm(AluOp::Set(3, true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbe0 => self.arith_imm(AluOp::Set(4, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbe1 => self.arith_imm(AluOp::Set(4, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbe2 => self.arith_imm(AluOp::Set(4, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbe3 => self.arith_imm(AluOp::Set(4, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbe4 => self.arith_imm(AluOp::Set(4, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbe5 => self.arith_imm(AluOp::Set(4, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbe6 => self.bitwise_hl_ptr(AluOp::Set(4, true), lookup::get_flags(opcode)),
            0xcbe7 => self.arith_imm(AluOp::Set(4, true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbe8 => self.arith_imm(AluOp::Set(5, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbe9 => self.arith_imm(AluOp::Set(5, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbea => self.arith_imm(AluOp::Set(5, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbeb => self.arith_imm(AluOp::Set(5, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbec => self.arith_imm(AluOp::Set(5, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbed => self.arith_imm(AluOp::Set(5, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbee => self.bitwise_hl_ptr(AluOp::Set(5, true), lookup::get_flags(opcode)),
            0xcbef => self.arith_imm(AluOp::Set(5, true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbf0 => self.arith_imm(AluOp::Set(6, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbf1 => self.arith_imm(AluOp::Set(6, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbf2 => self.arith_imm(AluOp::Set(6, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbf3 => self.arith_imm(AluOp::Set(6, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbf4 => self.arith_imm(AluOp::Set(6, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbf5 => self.arith_imm(AluOp::Set(6, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbf6 => self.bitwise_hl_ptr(AluOp::Set(6, true), lookup::get_flags(opcode)),
            0xcbf7 => self.arith_imm(AluOp::Set(6, true), Reg8::A, lookup::get_flags(opcode), 0),
            0xcbf8 => self.arith_imm(AluOp::Set(7, true), Reg8::B, lookup::get_flags(opcode), 0),
            0xcbf9 => self.arith_imm(AluOp::Set(7, true), Reg8::C, lookup::get_flags(opcode), 0),
            0xcbfa => self.arith_imm(AluOp::Set(7, true), Reg8::D, lookup::get_flags(opcode), 0),
            0xcbfb => self.arith_imm(AluOp::Set(7, true), Reg8::E, lookup::get_flags(opcode), 0),
            0xcbfc => self.arith_imm(AluOp::Set(7, true), Reg8::H, lookup::get_flags(opcode), 0),
            0xcbfd => self.arith_imm(AluOp::Set(7, true), Reg8::L, lookup::get_flags(opcode), 0),
            0xcbfe => self.bitwise_hl_ptr(AluOp::Set(7, true), lookup::get_flags(opcode)),
            0xcbff => self.arith_imm(AluOp::Set(7, true), Reg8::A, lookup::get_flags(opcode), 0),

            _ => {
                println!("Fatal error: undefined instruction! Opcode: 0x{:02x}", opcode);
                self.regs.print_registers();
                self.quit = true;
            }
        }

        // Print info about this instruction. Leaving this on all the time until the software
        // matures a little.
        self.print_instruction_info(&inst, old_pc);

        // Print register info if we have a breakpoint.
        if self.breaks.contains(&old_pc) || self.step {
            self.step = false;
            self.regs.print_registers();
            self.handle_breakpoint(old_pc);
        }

        !self.quit
    }

    fn print_instruction_info(&self, inst: &Instruction, old_pc: u16) {
        let mut pstr = format!("0x{:04x}: {} - {} cycles", old_pc, inst.name, inst.clocks);
        if inst.bytes > 1 && !inst.prefix_cb {
            pstr += " - operands: ";
            for i in 1..inst.bytes {
                pstr += &format!("0x{:02x} ", self.mem_get(old_pc + i as u16));
            }
        }
        println!("{}", pstr);
    }

    pub fn add_breakpoint(&mut self, addr: u16) {
        self.breaks.insert(addr);
    }

    fn handle_breakpoint(&mut self, addr: u16) {
        print!("Breaking at PC 0x{:04x}\nPress \'c\' to continue, \'n\' to step next: ", addr);
        let mut selection = String::new();
        io::stdout().flush().ok().expect("Problem flushing stdout.");
        io::stdin().read_line(&mut selection).expect("Could not read from stdin!");
        selection = selection.trim_matches(char::is_whitespace).to_string();
        if selection == "s" || selection == "n" {
            self.step = true;
        }
    }
}
