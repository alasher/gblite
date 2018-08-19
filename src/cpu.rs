#![allow(dead_code)]

use memory::Memory;
use lookup::Instruction;
use registers::RegisterCache;
use registers::Reg8;
use registers::Reg16;
use registers::RegOps;
use util;
use lookup;

enum AluOp {
    Add,
    AddCarry,
    Sub,
    SubCarry,
    And,
    Xor,
    Or,
    Comp
}

pub struct CPU {
    regs: RegisterCache,
    mem: Memory,
    ir_enabled: bool,
    quit: bool,
    jumped: bool
}

impl CPU {
    pub fn new(mem: Memory) -> CPU {
        CPU {
            regs: RegisterCache::new(),
            mem: mem,
            ir_enabled: true,
            quit: false,
            jumped: false,
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.regs.get(Reg16::PC)
    }

    // Get the u16 value starting at $(addr), little endian.
    fn parse_u16(&self, addr: u16) -> u16 {
        util::join_u8((self.mem.get(addr), self.mem.get(addr+1)))
    }

    // Push addr to stack
    fn push(&mut self, addr: u16) {
        let split_addr = util::split_u16(addr);
        let sp_val = self.regs.get(Reg16::SP)-2;
        self.regs.set(Reg16::SP, sp_val);
        self.mem.set(split_addr.0, sp_val);
        self.mem.set(split_addr.1, sp_val+1);
    }

    // Pop topmost u16 value from stack
    fn pop(&mut self) -> u16 {
        let sp_val = self.regs.get(Reg16::SP);
        let stack_val = self.parse_u16(sp_val);
        self.regs.set(Reg16::SP, sp_val+2);
        stack_val
    }

    // Push next_addr to stack, and jump to the jump_addr
    fn call(&mut self, jump_addr: u16) {
        let next_addr = self.regs.get(Reg16::PC);
        self.push(next_addr);
        self.regs.set(Reg16::PC, jump_addr);
        self.jumped = true;
    }

    // Pop the topmost address from the stack, and jump to it.
    fn ret(&mut self) {
        let next_addr = self.pop();
        self.regs.set(Reg16::PC, next_addr);
        self.jumped = true;
    }

    // Copy from given register into (HL).
    fn set_hl_ptr(&mut self, src: Reg8) {
        let val = self.regs.get(src);
        self.mem.set(val, self.regs.get(Reg16::HL));
    }

    // Copy value from (HL) into given register.
    fn get_hl_ptr(&mut self, dst: Reg8) {
        let val = self.mem.get(self.regs.get(Reg16::HL));
        self.regs.set(dst, val);
    }

    // Jump relative to current PC, where offset is twos-complement 8-bit signed int.
    fn jump_relative(&mut self, offset: u8) {
        let addr = self.regs.get(Reg16::PC) as i32;
        let addr = addr + (offset as i8) as i32;
        if addr < 0 || addr > 0xFFFF {
            println!("Fatal error: jumped out-of-bounds!");
            self.quit = true;
            return;
        }

        self.regs.set(Reg16::PC, addr as u16);
        self.jumped = true;
    }

    // Perform given ALU instruction with the given argument
    fn alu(&mut self, op: AluOp, val: u8) {
        let a = self.regs.get(Reg8::A);
        let cy = 0; // TODO: Retrieve this from register cache
        let result = match op {
            AluOp::Add      => a + val,
            AluOp::AddCarry => a + val + cy,
            AluOp::Sub      => a - val,
            AluOp::SubCarry => a - val - cy,
            AluOp::And      => a & val,
            AluOp::Xor      => a ^ val,
            AluOp::Or       => a | val,
            AluOp::Comp     => !val
        };

        self.regs.set(Reg8::A, result);

        // TODO: Add flag mods here
    }

    // For HALT, just exit the program for now. TODO: Add accurate HALT emulation here.
    fn halt(&mut self) {
        println!("Encountered HALT instruction, exiting!");
        self.quit = true;
    }

    // Run the instruction at the current PC, return true if successful.
    pub fn process(&mut self) -> bool {
        if self.quit { return false; }
        let old_pc = self.regs.get(Reg16::PC);
        let opcode = self.mem.get(old_pc);
        let _operand8  = self.mem.get(old_pc+1);
        let _operand16 = self.parse_u16(old_pc+1);
        self.jumped = false;

        // Adjust opcode if it's a 0xCB prefixed instruction
        let opcode = if opcode == 0xCB {
            let newop = 0xCB as u16 | _operand8 as u16;
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
            0x00 => (),
            0x01 => self.regs.set(Reg16::BC, _operand16),
            0x02 => {
                let r = self.regs.get(Reg8::A);
                self.mem.set(r, self.regs.get(Reg16::BC));
            },
            0x03 => {
                let val = self.regs.get(Reg16::BC) + 1;
                self.regs.set(Reg16::BC, val);
            },
            0x04 => {
                let val = self.regs.get(Reg8::B);
                self.regs.set(Reg8::B, val + 1);
            },
            0x05 => {
                let val = self.regs.get(Reg8::B);
                self.regs.set(Reg8::B, val - 1);
            },
            0x06 => self.regs.set(Reg8::B, _operand8),
            0x0A => {
                let r = self.mem.get(self.regs.get(Reg16::BC));
                self.regs.set(Reg8::A, r);
            },
            0x0B => {
                let val = self.regs.get(Reg16::BC) - 1;
                self.regs.set(Reg16::BC, val);
            },
            0x0C => {
                let val = self.regs.get(Reg8::C);
                self.regs.set(Reg8::C, val + 1);
            },
            0x0D => {
                let val = self.regs.get(Reg8::C);
                self.regs.set(Reg8::C, val - 1);
            },
            0x0E => self.regs.set(Reg8::C, _operand8),
            0x10 => {
                println!("Received STOP instruction, terminating.");
                self.quit = true;
            },
            0x11 => self.regs.set(Reg16::DE, _operand16),
            0x12 => {
                let r = self.regs.get(Reg8::A);
                self.mem.set(r, self.regs.get(Reg16::DE));
            },
            0x13 => {
                let val = self.regs.get(Reg16::DE) + 1;
                self.regs.set(Reg16::DE, val);
            },
            0x14 => {
                let val = self.regs.get(Reg8::D);
                self.regs.set(Reg8::D, val + 1);
            },
            0x15 => {
                let val = self.regs.get(Reg8::D);
                self.regs.set(Reg8::D, val - 1);
            },
            0x16 => self.regs.set(Reg8::D, _operand8),
            0x18 => self.jump_relative(_operand8),
            0x1B => {
                let val = self.regs.get(Reg16::DE) - 1;
                self.regs.set(Reg16::DE, val);
            },
            0x1A => {
                let r = self.mem.get(self.regs.get(Reg16::DE));
                self.regs.set(Reg8::A, r);
            },
            0x1C => {
                let val = self.regs.get(Reg8::E);
                self.regs.set(Reg8::E, val + 1);
            },
            0x1D => {
                let val = self.regs.get(Reg8::E);
                self.regs.set(Reg8::E, val - 1);
            },
            0x1E => self.regs.set(Reg8::E, _operand8),
            0x21 => self.regs.set(Reg16::HL, _operand16),
            0x22 => {
                let addr = self.regs.get(Reg16::HL);
                let r = self.regs.get(Reg8::A);
                self.mem.set(r, addr);
                self.regs.set(Reg16::HL, addr + 1);
            },
            0x23 => {
                let val = self.regs.get(Reg16::HL) + 1;
                self.regs.set(Reg16::HL, val);
            },
            0x24 => {
                let val = self.regs.get(Reg8::H);
                self.regs.set(Reg8::H, val + 1);
            },
            0x25 => {
                let val = self.regs.get(Reg8::H);
                self.regs.set(Reg8::H, val - 1);
            },
            0x26 => self.regs.set(Reg8::H, _operand8),
            0x2A => {
                let addr = self.regs.get(Reg16::HL);
                let r = self.mem.get(addr);
                self.regs.set(Reg8::A, r);
                self.regs.set(Reg16::HL, addr + 1);
            },
            0x2B => {
                let val = self.regs.get(Reg16::HL) - 1;
                self.regs.set(Reg16::HL, val);
            },
            0x2C => {
                let val = self.regs.get(Reg8::L);
                self.regs.set(Reg8::L, val + 1);
            },
            0x2D => {
                let val = self.regs.get(Reg8::L);
                self.regs.set(Reg8::L, val - 1);
            },
            0x2E => self.regs.set(Reg8::L, _operand8),
            0x31 => self.regs.set(Reg16::SP, _operand16),
            0x32 => {
                let addr = self.regs.get(Reg16::HL);
                let r = self.regs.get(Reg8::A);
                self.mem.set(r, addr);
                self.regs.set(Reg16::HL, addr - 1);
            },
            0x33 => {
                let sp_val = self.regs.get(Reg16::SP);
                self.regs.set(Reg16::SP, sp_val+1);
            },
            0x34 => {
                let addr = self.regs.get(Reg16::HL);
                let val = self.mem.get(addr);
                self.mem.set(val + 1, addr);
            },
            0x35 => {
                let addr = self.regs.get(Reg16::HL);
                let val = self.mem.get(addr);
                self.mem.set(val - 1, addr);
            },
            0x36 => self.mem.set(_operand8, self.regs.get(Reg16::HL)),
            0x3A => {
                let addr = self.regs.get(Reg16::HL);
                let r = self.mem.get(addr);
                self.regs.set(Reg8::A, r);
                self.regs.set(Reg16::HL, addr - 1);
            },
            0x3B => {
                let sp_val = self.regs.get(Reg16::SP);
                self.regs.set(Reg16::SP, sp_val-1);
            },
            0x3C => {
                let val = self.regs.get(Reg8::A);
                self.regs.set(Reg8::A, val + 1);
            },
            0x3D => {
                let val = self.regs.get(Reg8::A);
                self.regs.set(Reg8::A, val - 1);
            },
            0x3E => self.regs.set(Reg8::A, _operand8),

            // [0x40, 0x7F] - Mostly copy instructions between registers and (HL).
            0x40 => self.regs.copy(Reg8::B, Reg8::B),
            0x41 => self.regs.copy(Reg8::B, Reg8::C),
            0x42 => self.regs.copy(Reg8::B, Reg8::D),
            0x43 => self.regs.copy(Reg8::B, Reg8::E),
            0x44 => self.regs.copy(Reg8::B, Reg8::H),
            0x45 => self.regs.copy(Reg8::B, Reg8::L),
            0x46 => self.get_hl_ptr(Reg8::B),
            0x47 => self.regs.copy(Reg8::B, Reg8::A),
            0x48 => self.regs.copy(Reg8::C, Reg8::B),
            0x49 => self.regs.copy(Reg8::C, Reg8::C),
            0x4a => self.regs.copy(Reg8::C, Reg8::D),
            0x4b => self.regs.copy(Reg8::C, Reg8::E),
            0x4c => self.regs.copy(Reg8::C, Reg8::H),
            0x4d => self.regs.copy(Reg8::C, Reg8::L),
            0x4e => self.get_hl_ptr(Reg8::C),
            0x4f => self.regs.copy(Reg8::C, Reg8::A),
            0x50 => self.regs.copy(Reg8::D, Reg8::B),
            0x51 => self.regs.copy(Reg8::D, Reg8::C),
            0x52 => self.regs.copy(Reg8::D, Reg8::D),
            0x53 => self.regs.copy(Reg8::D, Reg8::E),
            0x54 => self.regs.copy(Reg8::D, Reg8::H),
            0x55 => self.regs.copy(Reg8::D, Reg8::L),
            0x56 => self.get_hl_ptr(Reg8::D),
            0x57 => self.regs.copy(Reg8::D, Reg8::A),
            0x58 => self.regs.copy(Reg8::E, Reg8::B),
            0x59 => self.regs.copy(Reg8::E, Reg8::C),
            0x5a => self.regs.copy(Reg8::E, Reg8::D),
            0x5b => self.regs.copy(Reg8::E, Reg8::E),
            0x5c => self.regs.copy(Reg8::E, Reg8::H),
            0x5d => self.regs.copy(Reg8::E, Reg8::L),
            0x5e => self.get_hl_ptr(Reg8::E),
            0x5f => self.regs.copy(Reg8::E, Reg8::A),
            0x60 => self.regs.copy(Reg8::H, Reg8::B),
            0x61 => self.regs.copy(Reg8::H, Reg8::C),
            0x62 => self.regs.copy(Reg8::H, Reg8::D),
            0x63 => self.regs.copy(Reg8::H, Reg8::E),
            0x64 => self.regs.copy(Reg8::H, Reg8::H),
            0x65 => self.regs.copy(Reg8::H, Reg8::L),
            0x66 => self.get_hl_ptr(Reg8::H),
            0x67 => self.regs.copy(Reg8::H, Reg8::A),
            0x68 => self.regs.copy(Reg8::L, Reg8::B),
            0x69 => self.regs.copy(Reg8::L, Reg8::C),
            0x6a => self.regs.copy(Reg8::L, Reg8::D),
            0x6b => self.regs.copy(Reg8::L, Reg8::E),
            0x6c => self.regs.copy(Reg8::L, Reg8::H),
            0x6d => self.regs.copy(Reg8::L, Reg8::L),
            0x6e => self.get_hl_ptr(Reg8::L),
            0x6f => self.regs.copy(Reg8::L, Reg8::A),
            0x70 => self.set_hl_ptr(Reg8::B),
            0x71 => self.set_hl_ptr(Reg8::C),
            0x72 => self.set_hl_ptr(Reg8::D),
            0x73 => self.set_hl_ptr(Reg8::E),
            0x74 => self.set_hl_ptr(Reg8::H),
            0x75 => self.set_hl_ptr(Reg8::L),
            0x76 => self.halt(),
            0x77 => self.set_hl_ptr(Reg8::A),
            0x78 => self.regs.copy(Reg8::A, Reg8::B),
            0x79 => self.regs.copy(Reg8::A, Reg8::C),
            0x7a => self.regs.copy(Reg8::A, Reg8::D),
            0x7b => self.regs.copy(Reg8::A, Reg8::E),
            0x7c => self.regs.copy(Reg8::A, Reg8::H),
            0x7d => self.regs.copy(Reg8::A, Reg8::L),
            0x7e => self.get_hl_ptr(Reg8::A),
            0x7f => self.regs.copy(Reg8::A, Reg8::A),
            0xC1 => {
                let val = self.pop();
                self.regs.set(Reg16::BC, val);
            },
            0xC3 => {
                self.regs.set(Reg16::PC, _operand16);
                self.jumped = true;
            },
            0xC5 => {
                let reg = self.regs.get(Reg16::BC);
                self.push(reg);
            },
            0xC7 => {
                self.call(0x00);
            },
            0xC9 => self.ret(),
            0xCB => self.quit = true, // This shouldn't ever happen
            0xCD => self.call(_operand16),
            0xCF => self.call(0x08),
            0xD1 => {
                let val = self.pop();
                self.regs.set(Reg16::DE, val);
            },
            0xD5 => {
                let reg = self.regs.get(Reg16::DE);
                self.push(reg);
            },
            0xD7 => self.call(0x10),
            0xD9 => {
                self.ret();
                self.ir_enabled = true;
            },
            0xDF => self.call(0x18),
            0xE0 => self.mem.set(self.regs.get(Reg8::A), 0xFF00 + (_operand8 as u16)),
            0xE1 => {
                let val = self.pop();
                self.regs.set(Reg16::HL, val);
            },
            0xE2 => {
                let addr = 0xFF00 + self.regs.get(Reg8::C) as u16;
                self.mem.set(self.regs.get(Reg8::A), addr);
            },
            0xE7 => self.call(0x20),
            0xEA => self.mem.set(self.regs.get(Reg8::A), _operand16),
            0xE5 => {
                let reg = self.regs.get(Reg16::HL);
                self.push(reg);
            },
            0xEF => self.call(0x28),
            0xF0 => self.regs.set(Reg8::A, self.mem.get(0xFF00 + (_operand8 as u16))),
            0xF1 => {
                let val = self.pop();
                self.regs.set(Reg16::AF, val);
            },
            0xF2 => {
                let addr = 0xFF00 + self.regs.get(Reg8::C) as u16;
                self.mem.set(self.regs.get(Reg8::A), addr);
            },
            0xF3 => self.ir_enabled = false,
            0xFA => self.regs.set(Reg8::A, self.mem.get(_operand16)),
            0xFB => self.ir_enabled = true,
            0xF5 => {
                let reg = self.regs.get(Reg16::AF);
                self.push(reg);
            },
            0xF7 => self.call(0x30),
            0xFF => self.call(0x38),
            _ => {
                println!("Fatal error: undefined instruction!");
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
