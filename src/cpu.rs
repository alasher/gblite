#![allow(dead_code)]

use memory::Memory;
use lookup::Instruction;
use registers::*;
use util;
use lookup;

use std::fmt;

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
    pub mem: Memory,
    pub ir_enabled: bool,
    quit: bool,
    was_zero: bool,
    half_carry: bool,
    full_carry: bool
}

impl CPU {
    pub fn new(mem: Memory) -> CPU {
        CPU {
            regs: RegisterCache::new(),
            mem: mem,
            ir_enabled: true,
            quit: false,
            was_zero: false,
            half_carry: false,
            full_carry: false
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.regs.get(Reg16::PC)
    }

    // Get the u16 value starting at $(addr), little endian.
    fn parse_u16(&self, addr: u16) -> u16 {
        util::join_u8((self.mem.get(addr), self.mem.get(addr+1)))
    }

    // Push addr from given register onto stack
    fn push(&mut self, src: Reg16) {
        self.regs.sub(Reg16::SP, 2);
        let sp_val = self.regs.get(Reg16::SP);
        let split_addr = util::split_u16(self.regs.get(src));
        self.mem.set(split_addr.0, sp_val);
        self.mem.set(split_addr.1, sp_val+1);
    }

    // Pop topmost u16 value from stack, store to given register
    fn pop(&mut self, dst: Reg16) {
        let stack_val = self.parse_u16(self.regs.get(Reg16::SP));
        self.regs.add(Reg16::SP, 2);
        self.regs.set(dst, stack_val);
    }

    fn call_flag(&mut self, flag: Flag, if_unset: bool, addr: u16) {
        let flag_val = match flag {
            Flag::Z | Flag::CY => self.regs.get_flag(flag),
            _ => panic!("CALL for flag only exists for Z and CY flags!")
        };

        if flag_val ^ if_unset {
            self.call(addr);
        }
    }

    // Push next_addr to stack, and jump to the jump_addr
    fn call(&mut self, jump_addr: u16) {
        self.push(Reg16::PC);
        self.regs.set(Reg16::PC, jump_addr);
    }

    // Call ret if flag matches
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
        let val = self.regs.get(src);
        self.mem.set(val, self.regs.get(dst));
    }

    // Copy value from (HL) into given register.
    fn get_reg_ptr(&mut self, dst: Reg8, src: Reg16) {
        let val = self.mem.get(self.regs.get(src));
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

    fn ld_fast_page(&mut self, is_get: bool) {
        let addr = 0xff00 + self.regs.get(Reg8::C) as u16;
        if is_get {
            self.regs.set(Reg8::A, self.mem.get(addr));
        } else {
            self.mem.set(self.regs.get(Reg8::A), addr);
        }
    }

    // Increment/decrement for (HL) value. TODO: should this be done another way? Maybe implement
    // it as a Reg8::HL_PTR, or something special in ALU?
    fn hl_ptr_inc_dec(&mut self, is_add: bool) {
        let addr = self.regs.get(Reg16::HL);
        let val = self.mem.get(addr);
        let val = if is_add { val + 1 } else { val - 1};
        self.mem.set(val, addr);
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
            }
            _ => panic!("Unimplemented ALU function!")
        };

        if op != AluOp::Comp {
            self.was_zero = result == 0;
        }

        if cfg!(debug_assertions) {
            println!("Result of ALU instruction {} with input {}, {} => {}. Z: {}, H: {}, CY: {}",
                     op, op_a, op_b, result, self.was_zero, self.half_carry, self.full_carry);
        }

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
        self.arith_imm(op, flags, operand_b);
    }

    fn arith_hl_ptr(&mut self, op: AluOp, flags: FlagStatus) {
        let operand_b = self.regs.get(Reg16::HL);
        let operand_b = self.mem.get(operand_b);
        self.arith_imm(op, flags, operand_b);
    }

    // Take an immediate u8 instead of a register.
    fn arith_imm(&mut self, op: AluOp, flags: FlagStatus, val: u8) {
        let operand_a = self.regs.get(Reg8::A);
        let result = self.alu(op, operand_a, val);
        self.regs.set(Reg8::A, result);
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

    // For HALT, just exit the program for now. TODO: Add accurate HALT emulation here.
    fn halt(&mut self) {
        println!("Encountered HALT instruction, exiting!");
        self.quit = true;
    }

    fn stop(&mut self) {
        println!("Encountered STOP instruction, exiting!");
        self.quit = true;
    }

    // Run the instruction at the current PC, return true if successful.
    pub fn process(&mut self) -> bool {
        if self.quit { return false; }
        let old_pc = self.regs.get(Reg16::PC);
        let opcode = self.mem.get(old_pc);
        let _operand8  = self.mem.get(old_pc+1);
        let _operand16 = self.parse_u16(old_pc+1);

        // Adjust opcode if it's a 0xcb prefixed instruction
        let opcode = if opcode == 0xcb {
            let newop = ((0xcb as u16) << 8) | _operand8 as u16;
            let _operand8  = self.mem.get(old_pc+2);
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

        // Print info about this instruction. Leaving this on all the time until the software
        // matures a little. development
        self.print_instruction_info(&inst, old_pc);

        match opcode {
            // [0x00, 0x3f] - Load, INC/DEC, some jumps, and other various instructions.
            0x00 => (),
            0x01 => self.regs.set(Reg16::BC, _operand16),
            0x02 => self.set_reg_ptr(Reg16::BC, Reg8::A),
            0x03 => self.regs.add(Reg16::BC, 1),
            0x04 => self.regs.add(Reg8::B, 1),
            0x05 => self.regs.sub(Reg8::B, 1),
            0x06 => self.regs.set(Reg8::B, _operand8),
            0x09 => self.add_hl(lookup::get_flags(opcode), Reg16::BC),
            0x0a => self.get_reg_ptr(Reg8::A, Reg16::BC),
            0x0b => self.regs.sub(Reg16::BC, 1),
            0x0c => self.regs.add(Reg8::C, 1),
            0x0d => self.regs.sub(Reg8::C, 1),
            0x0e => self.regs.set(Reg8::C, _operand8),
            0x10 => self.stop(),
            0x11 => self.regs.set(Reg16::DE, _operand16),
            0x12 => self.set_reg_ptr(Reg16::DE, Reg8::A),
            0x13 => self.regs.add(Reg16::DE, 1),
            0x14 => self.regs.add(Reg8::D, 1),
            0x15 => self.regs.sub(Reg8::D, 1),
            0x16 => self.regs.set(Reg8::D, _operand8),
            0x18 => self.jump_relative(_operand8),
            0x19 => self.add_hl(lookup::get_flags(opcode), Reg16::DE),
            0x1a => self.get_reg_ptr(Reg8::A, Reg16::DE),
            0x1b => self.regs.sub(Reg16::BC, 1),
            0x1c => self.regs.add(Reg8::D, 1),
            0x1d => self.regs.sub(Reg8::D, 1),
            0x1e => self.regs.set(Reg8::E, _operand8),
            0x20 => self.jump_relative_flag(Flag::Z, true, _operand8),
            0x21 => self.regs.set(Reg16::HL, _operand16),
            0x22 => self.ldd_special(true, true),
            0x23 => self.regs.add(Reg16::HL, 1),
            0x24 => self.regs.add(Reg8::H, 1),
            0x25 => self.regs.sub(Reg8::H, 1),
            0x26 => self.regs.set(Reg8::H, _operand8),
            0x28 => self.jump_relative_flag(Flag::Z, false, _operand8),
            0x29 => self.add_hl(lookup::get_flags(opcode), Reg16::HL),
            0x2a => self.ldd_special(false, true),
            0x2b => self.regs.sub(Reg16::HL, 1),
            0x2c => self.regs.add(Reg8::L, 1),
            0x2d => self.regs.sub(Reg8::L, 1),
            0x2e => self.regs.set(Reg8::L, _operand8),
            // 0x30 => self.jump_relative_flag(Flag::CY, true, _operand8),
            0x31 => self.regs.set(Reg16::SP, _operand16),
            0x32 => self.ldd_special(true, false),
            0x33 => self.regs.add(Reg16::HL, 1),
            0x34 => self.hl_ptr_inc_dec(true),
            0x35 => self.hl_ptr_inc_dec(false),
            0x36 => self.mem.set(_operand8, self.regs.get(Reg16::HL)),
            // 0x38 => self.jump_relative_flag(Flag::CY, false, _operand8),
            0x39 => self.add_hl(lookup::get_flags(opcode), Reg16::SP),
            0x3a => self.ldd_special(false, false),
            0x3b => self.regs.sub(Reg16::SP, 1),
            0x3c => self.regs.add(Reg8::A, 1),
            0x3d => self.regs.sub(Reg8::A, 1),
            0x3e => self.regs.set(Reg8::A, _operand8),

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
            0xc6 => self.arith_imm(AluOp::Add(false), lookup::get_flags(opcode), _operand8),
            0xc7 => self.call(0x00),
            0xc8 => self.ret_flag(Flag::Z, false),
            0xc9 => self.ret(false),
            0xca => self.jump_flag(Flag::Z, false, _operand16),
            0xcb => self.quit = true, // This shouldn't ever happen
            0xcc => self.call_flag(Flag::Z, false, _operand16),
            0xcd => self.call(_operand16),
            0xce => self.arith_imm(AluOp::Add(true), lookup::get_flags(opcode), _operand8),
            0xcf => self.call(0x08),
            // 0xd0 => self.ret_flag(Flag::CY, true), TODO: Fix CY flag support
            0xd1 => self.pop(Reg16::DE),
            // 0xd2 => self.jump_flag(Flag::CY, true, _operand16),
            // 0xd4 => self.call_flag(Flag::CY, true, _operand16),
            0xd5 => self.push(Reg16::DE),
            0xd6 => self.arith_imm(AluOp::Sub(false), lookup::get_flags(opcode), _operand8),
            0xd7 => self.call(0x10),
            // 0xd8 => self.ret_flag(Flag::CY, false), TODO: Fix CY flag support
            0xd9 => self.ret(true),
            // 0xda => self.jump_flag(Flag::CY, false, _operand16),
            // 0xdc => self.call_flag(Flag::CY, false, _operand16),
            0xde => self.arith_imm(AluOp::Sub(true), lookup::get_flags(opcode), _operand8),
            0xdf => self.call(0x18),
            0xe0 => self.mem.set(self.regs.get(Reg8::A), 0xff00 + (_operand8 as u16)),
            0xe1 => self.pop(Reg16::HL),
            0xe2 => self.ld_fast_page(true),
            0xe5 => self.push(Reg16::HL),
            0xe6 => self.arith_imm(AluOp::And, lookup::get_flags(opcode), _operand8),
            0xe7 => self.call(0x20),
            0xea => self.mem.set(self.regs.get(Reg8::A), _operand16),
            0xee => self.arith_imm(AluOp::Xor, lookup::get_flags(opcode), _operand8),
            0xef => self.call(0x28),
            0xf0 => self.regs.set(Reg8::A, self.mem.get(0xff00 + (_operand8 as u16))),
            0xf1 => self.pop(Reg16::AF),
            0xf2 => self.ld_fast_page(false),
            0xf3 => self.ir_enabled = false,
            0xf5 => self.push(Reg16::AF),
            0xf6 => self.arith_imm(AluOp::Or, lookup::get_flags(opcode), _operand8),
            0xf7 => self.call(0x30),
            0xfa => self.regs.set(Reg8::A, self.mem.get(_operand16)),
            0xfb => self.ir_enabled = true,
            0xfe => self.arith_imm(AluOp::Comp, lookup::get_flags(opcode), _operand8),
            0xff => self.call(0x38),
            _ => {
                println!("Fatal error: undefined instruction! Opcode: 0x{:02x}", opcode);
                self.regs.print_registers();
                self.quit = true;
            }
        }

        !self.quit
    }

    fn print_instruction_info(&self, inst: &Instruction, old_pc: u16) {
        let mut pstr = format!("0x{:04x}: {} - {} cycles", old_pc, inst.name, inst.clocks);
        if inst.bytes > 1 {
            pstr += " - operands: ";
            for i in 1..inst.bytes {
                pstr += &format!("0x{:02x} ", self.mem.get(old_pc + i as u16));
            }
        }
        println!("{}", pstr);
    }
}
