#![allow(dead_code)]

use memory::Memory;
use lookup::Instruction;
use registers::RegisterCache;
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
        let a = self.regs.get_a();
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

        self.regs.set_a(result);

        // TODO: Add flag mods here
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


        match opcode {
            0x00 => (),
            0x01 => {
                self.regs.set_bc(_operand16);
            },
            0x02 => {
                let r = self.regs.get_a();
                self.mem.set(r, self.regs.get_bc());
            },
            0x03 => {
                let val = self.regs.get_bc() + 1;
                self.regs.set_bc(val);
            },
            0x04 => {
                let val = self.regs.get_b();
                self.regs.set_b(val + 1);
            },
            0x05 => {
                let val = self.regs.get_b();
                self.regs.set_b(val - 1);
            },
            0x06 => {
                self.regs.set_b(_operand8);
            },
            0x0A => {
                let r = self.mem.get(self.regs.get_bc());
                self.regs.set_a(r);
            },
            0x0B => {
                let val = self.regs.get_bc() - 1;
                self.regs.set_bc(val);
            },
            0x0C => {
                let val = self.regs.get_c();
                self.regs.set_c(val + 1);
            },
            0x0D => {
                let val = self.regs.get_c();
                self.regs.set_c(val - 1);
            },
            0x0E => {
                self.regs.set_c(_operand8);
            },
            0x10 => {
                println!("Received STOP instruction, terminating.");
                self.quit = true;
            },
            0x11 => {
                self.regs.set_de(_operand16);
            },
            0x12 => {
                let r = self.regs.get_a();
                self.mem.set(r, self.regs.get_de());
            },
            0x13 => {
                let val = self.regs.get_de() + 1;
                self.regs.set_de(val);
            },
            0x14 => {
                let val = self.regs.get_d();
                self.regs.set_d(val + 1);
            },
            0x15 => {
                let val = self.regs.get_d();
                self.regs.set_d(val - 1);
            },
            0x16 => {
                self.regs.set_d(_operand8);
            },
            0x18 => {
                self.jump_relative(_operand8);
            },
            0x1B => {
                let val = self.regs.get_de() - 1;
                self.regs.set_de(val);
            },
            0x1A => {
                let r = self.mem.get(self.regs.get_de());
                self.regs.set_a(r);
            },
            0x1C => {
                let val = self.regs.get_e();
                self.regs.set_e(val + 1);
            },
            0x1D => {
                let val = self.regs.get_e();
                self.regs.set_e(val - 1);
            },
            0x1E => {
                self.regs.set_e(_operand8);
            },
            0x21 => {
                self.regs.set_hl(_operand16);
            },
            0x22 => {
                let addr = self.regs.get_hl();
                let r = self.regs.get_a();
                self.mem.set(r, addr);
                self.regs.set_hl(addr + 1);
            },
            0x23 => {
                let val = self.regs.get_hl() + 1;
                self.regs.set_hl(val);
            },
            0x24 => {
                let val = self.regs.get_h();
                self.regs.set_h(val + 1);
            },
            0x25 => {
                let val = self.regs.get_h();
                self.regs.set_h(val - 1);
            },
            0x26 => {
                self.regs.set_h(_operand8);
            },
            0x2A => {
                let addr = self.regs.get_hl();
                let r = self.mem.get(addr);
                self.regs.set_a(r);
                self.regs.set_hl(addr + 1);
            },
            0x2B => {
                let val = self.regs.get_hl() - 1;
                self.regs.set_hl(val);
            },
            0x2C => {
                let val = self.regs.get_l();
                self.regs.set_l(val + 1);
            },
            0x2D => {
                let val = self.regs.get_l();
                self.regs.set_l(val - 1);
            },
            0x2E => {
                self.regs.set_l(_operand8);
            },
            0x31 => {
                self.regs.set(Reg16::SP, _operand16);
            },
            0x32 => {
                let addr = self.regs.get_hl();
                let r = self.regs.get_a();
                self.mem.set(r, addr);
                self.regs.set_hl(addr - 1);
            },
            0x33 => {
                let sp_val = self.regs.get(Reg16::SP);
                self.regs.set(Reg16::SP, sp_val+1);
            },
            0x34 => {
                let addr = self.regs.get_hl();
                let val = self.mem.get(addr);
                self.mem.set(val + 1, addr);
            },
            0x35 => {
                let addr = self.regs.get_hl();
                let val = self.mem.get(addr);
                self.mem.set(val - 1, addr);
            },
            0x36 => {
                self.mem.set(_operand8, self.regs.get_hl());
            },
            0x3A => {
                let addr = self.regs.get_hl();
                let r = self.mem.get(addr);
                self.regs.set_a(r);
                self.regs.set_hl(addr - 1);
            },
            0x3B => {
                let sp_val = self.regs.get(Reg16::SP);
                self.regs.set(Reg16::SP, sp_val-1);
            },
            0x3C => {
                let val = self.regs.get_a();
                self.regs.set_a(val + 1);
            },
            0x3D => {
                let val = self.regs.get_a();
                self.regs.set_a(val - 1);
            },
            0x3E => {
                self.regs.set_a(_operand8);
            },
            0x40 => {
                let r = self.regs.get_b();
                self.regs.set_b(r);
            },
            0x41 => {
                let r = self.regs.get_c();
                self.regs.set_b(r);
            },
            0x42 => {
                let r = self.regs.get_d();
                self.regs.set_b(r);
            },
            0x43 => {
                let r = self.regs.get_e();
                self.regs.set_b(r);
            },
            0x44 => {
                let r = self.regs.get_h();
                self.regs.set_b(r);
            },
            0x45 => {
                let r = self.regs.get_l();
                self.regs.set_b(r);
            },
            0x46 => {
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_b(r);
            },
            0x47 => {
                let r = self.regs.get_a();
                self.regs.set_b(r);
            },
            0x48 => {
                let r = self.regs.get_b();
                self.regs.set_c(r);
            },
            0x49 => {
                let r = self.regs.get_c();
                self.regs.set_c(r);
            },
            0x4a => {
                let r = self.regs.get_d();
                self.regs.set_c(r);
            },
            0x4b => {
                let r = self.regs.get_e();
                self.regs.set_c(r);
            },
            0x4c => {
                let r = self.regs.get_h();
                self.regs.set_c(r);
            },
            0x4d => {
                let r = self.regs.get_l();
                self.regs.set_c(r);
            },
            0x4e => {
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_c(r);
            },
            0x4f => {
                let r = self.regs.get_a();
                self.regs.set_c(r);
            },
            0x50 => {
                let r = self.regs.get_b();
                self.regs.set_d(r);
            },
            0x51 => {
                let r = self.regs.get_c();
                self.regs.set_d(r);
            },
            0x52 => {
                let r = self.regs.get_d();
                self.regs.set_d(r);
            },
            0x53 => {
                let r = self.regs.get_e();
                self.regs.set_d(r);
            },
            0x54 => {
                let r = self.regs.get_h();
                self.regs.set_d(r);
            },
            0x55 => {
                let r = self.regs.get_l();
                self.regs.set_d(r);
            },
            0x56 => {
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_d(r);
            },
            0x57 => {
                let r = self.regs.get_a();
                self.regs.set_d(r);
            },
            0x58 => {
                let r = self.regs.get_b();
                self.regs.set_e(r);
            },
            0x59 => {
                let r = self.regs.get_c();
                self.regs.set_e(r);
            },
            0x5a => {
                let r = self.regs.get_d();
                self.regs.set_e(r);
            },
            0x5b => {
                let r = self.regs.get_e();
                self.regs.set_e(r);
            },
            0x5c => {
                let r = self.regs.get_h();
                self.regs.set_e(r);
            },
            0x5d => {
                let r = self.regs.get_l();
                self.regs.set_e(r);
            },
            0x5e => {
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_e(r);
            },
            0x5f => {
                let r = self.regs.get_a();
                self.regs.set_e(r);
            },
            0x60 => {
                let r = self.regs.get_b();
                self.regs.set_h(r);
            },
            0x61 => {
                let r = self.regs.get_c();
                self.regs.set_h(r);
            },
            0x62 => {
                let r = self.regs.get_d();
                self.regs.set_h(r);
            },
            0x63 => {
                let r = self.regs.get_e();
                self.regs.set_h(r);
            },
            0x64 => {
                let r = self.regs.get_h();
                self.regs.set_h(r);
            },
            0x65 => {
                let r = self.regs.get_l();
                self.regs.set_h(r);
            },
            0x66 => {
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_h(r);
            },
            0x67 => {
                let r = self.regs.get_a();
                self.regs.set_h(r);
            },
            0x68 => {
                let r = self.regs.get_b();
                self.regs.set_l(r);
            },
            0x69 => {
                let r = self.regs.get_c();
                self.regs.set_l(r);
            },
            0x6a => {
                let r = self.regs.get_d();
                self.regs.set_l(r);
            },
            0x6b => {
                let r = self.regs.get_e();
                self.regs.set_l(r);
            },
            0x6c => {
                let r = self.regs.get_h();
                self.regs.set_l(r);
            },
            0x6d => {
                let r = self.regs.get_l();
                self.regs.set_l(r);
            },
            0x6e => {
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_l(r);
            },
            0x6f => {
                let r = self.regs.get_a();
                self.regs.set_l(r);
            },
            0x70 => {
                let r = self.regs.get_b();
                self.mem.set(r, self.regs.get_hl());
            },
            0x71 => {
                let r = self.regs.get_c();
                self.mem.set(r, self.regs.get_hl());
            },
            0x72 => {
                let r = self.regs.get_d();
                self.mem.set(r, self.regs.get_hl());
            },
            0x73 => {
                let r = self.regs.get_e();
                self.mem.set(r, self.regs.get_hl());
            },
            0x74 => {
                let r = self.regs.get_h();
                self.mem.set(r, self.regs.get_hl());
            },
            0x75 => {
                let r = self.regs.get_l();
                self.mem.set(r, self.regs.get_hl());
            },
            0x76 => {
                println!("Encountered HALT instruction, exiting!");
                self.quit = true;
            },
            0x77 => {
                let r = self.regs.get_a();
                self.mem.set(r, self.regs.get_hl());
            },
            0x78 => {
                let r = self.regs.get_b();
                self.regs.set_a(r);
            },
            0x79 => {
                let r = self.regs.get_c();
                self.regs.set_a(r);
            },
            0x7a => {
                let r = self.regs.get_d();
                self.regs.set_a(r);
            },
            0x7b => {
                let r = self.regs.get_e();
                self.regs.set_a(r);
            },
            0x7c => {
                let r = self.regs.get_h();
                self.regs.set_a(r);
            },
            0x7d => {
                let r = self.regs.get_l();
                self.regs.set_a(r);
            },
            0x7e => {
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_a(r);
            },
            0x7f => {
                let r = self.regs.get_a();
                self.regs.set_a(r);
            },
            0xC1 => {
                let val = self.pop();
                self.regs.set_bc(val);
            },
            0xC3 => {
                self.regs.set(Reg16::PC, _operand16);
                self.jumped = true;
            },
            0xC5 => {
                let reg = self.regs.get_bc();
                self.push(reg);
            },
            0xC7 => {
                self.call(0x00);
            },
            0xC9 => {
                self.ret();
            },
            0xCB => {
                // This should never happen, we should always append the prefix after CB, ex: 0xCB01
                println!("Fatal error: encountered unadjusted 0xCB literal!");
                self.quit = true;
            },
            0xCD => {
                self.call(_operand16);
            },
            0xCF => {
                self.call(0x08);
            },
            0xD1 => {
                let val = self.pop();
                self.regs.set_de(val);
            },
            0xD5 => {
                let reg = self.regs.get_de();
                self.push(reg);
            },
            0xD7 => {
                self.call(0x10);
            },
            0xD9 => {
                self.ret();
                self.ir_enabled = true;
            },
            0xDF => {
                self.call(0x18);
            },
            0xE0 => {
                self.mem.set(self.regs.get_a(), 0xFF00 + (_operand8 as u16));
            },
            0xE1 => {
                let val = self.pop();
                self.regs.set_hl(val);
            },
            0xE2 => {
                let addr = 0xFF00 + self.regs.get_c() as u16;
                self.mem.set(self.regs.get_a(), addr);
            },
            0xE7 => {
                self.call(0x20);
            },
            0xEA => {
                self.mem.set(self.regs.get_a(), _operand16);
            },
            0xE5 => {
                let reg = self.regs.get_hl();
                self.push(reg);
            },
            0xEF => {
                self.call(0x28);
            },
            0xF0 => {
                self.regs.set_a(self.mem.get(0xFF00 + (_operand8 as u16)));
            },
            0xF1 => {
                let val = self.pop();
                self.regs.set_af(val);
            },
            0xF2 => {
                let addr = 0xFF00 + self.regs.get_c() as u16;
                self.mem.set(self.regs.get_a(), addr);
            },
            0xF3 => {
                self.ir_enabled = false;
            },
            0xFA => {
                self.regs.set_a(self.mem.get(_operand16));
            },
            0xFB => {
                self.ir_enabled = true;
            },
            0xF5 => {
                let reg = self.regs.get_af();
                self.push(reg);
            },
            0xF7 => {
                self.call(0x30);
            },
            0xFF => {
                self.call(0x38);
            },
            _ => {
                println!("Fatal error: undefined instruction!");
                self.quit = true;
            }
        }

        self.print_instruction_info(&inst, old_pc);

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
