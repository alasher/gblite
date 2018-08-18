#![allow(dead_code)]

use memory::Memory;
use lookup::Instruction;
use util;
use lookup;

pub enum Reg {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

struct DoubleRegister {
    a: u8,
    b: u8
}

impl DoubleRegister {
    fn get_first(&self) -> u8 { self.a }
    fn set_first(&mut self, val: u8) { self.a = val; }
    fn get_second(&self) -> u8 { self.b }
    fn set_second(&mut self, val: u8) { self.b = val; }

    fn get_double(&self) -> u16 {
        (((self.get_first() as u16) << 8) | self.get_second() as u16)
    }

    // BC <- 0xFF11 means (B = 0xFF, C = 0x11)
    fn set_double(&mut self, val: u16) {
        self.a = (val >> 8)   as u8;
        self.b = (val & 0xFF) as u8;
    }
}

struct RegisterCache {
    af: DoubleRegister,
    bc: DoubleRegister,
    de: DoubleRegister,
    hl: DoubleRegister,
    sp: u16,
    pc: u16
}

impl RegisterCache {
    pub fn new() -> RegisterCache {
        RegisterCache {
            af: DoubleRegister{a: 0x0, b: 0x0},
            bc: DoubleRegister{a: 0x0, b: 0x0},
            de: DoubleRegister{a: 0x0, b: 0x0},
            hl: DoubleRegister{a: 0x0, b: 0x0},
            sp: 0x0,
            pc: 0x0
        }
    }

    // Get/Set for 8-bit registers
    pub fn get_a(&self) -> u8 { self.af.get_first() }
    pub fn get_b(&self) -> u8 { self.bc.get_first() }
    pub fn get_c(&self) -> u8 { self.bc.get_second() }
    pub fn get_d(&self) -> u8 { self.de.get_first() }
    pub fn get_e(&self) -> u8 { self.de.get_second() }
    pub fn get_h(&self) -> u8 { self.hl.get_first() }
    pub fn get_l(&self) -> u8 { self.hl.get_second() }
    pub fn set_a(&mut self, val: u8) { self.af.set_first(val); }
    pub fn set_b(&mut self, val: u8) { self.bc.set_first(val); }
    pub fn set_c(&mut self, val: u8) { self.bc.set_second(val); }
    pub fn set_d(&mut self, val: u8) { self.de.set_first(val); }
    pub fn set_e(&mut self, val: u8) { self.de.set_second(val); }
    pub fn set_h(&mut self, val: u8) { self.hl.set_first(val); }
    pub fn set_l(&mut self, val: u8) { self.hl.set_second(val); }

    // Get/Set for 16-bit registers
    pub fn get_af(&self) -> u16 { self.af.get_double() }
    pub fn get_bc(&self) -> u16 { self.bc.get_double() }
    pub fn get_de(&self) -> u16 { self.de.get_double() }
    pub fn get_hl(&self) -> u16 { self.hl.get_double() }
    pub fn set_af(&mut self, val: u16) { self.bc.set_double(val); }
    pub fn set_bc(&mut self, val: u16) { self.bc.set_double(val); }
    pub fn set_de(&mut self, val: u16) { self.de.set_double(val); }
    pub fn set_hl(&mut self, val: u16) { self.hl.set_double(val); }

    // Getters/setters for Flags
    fn get_flag(&self, fid: u8) -> bool {
        util::is_bit_set(self.af.b, fid)
    }
    fn set_flag(&mut self, val: bool, fid: u8) {
        self.af.b = util::set_bit(self.af.b, fid, val);
    }
    pub fn get_z(&self)  -> bool { self.get_flag(7) }
    pub fn get_n(&self)  -> bool { self.get_flag(6) }
    pub fn get_hc(&self)  -> bool { self.get_flag(5) }
    pub fn get_cy(&self) -> bool { self.get_flag(4) }
    pub fn set_z(&mut self,  val: bool) { self.set_flag(val, 7); }
    pub fn set_n(&mut self,  val: bool) { self.set_flag(val, 6); }
    pub fn set_hc(&mut self,  val: bool) { self.set_flag(val, 5); }
    pub fn set_cy(&mut self, val: bool) { self.set_flag(val, 4); }

}

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
        self.regs.pc
    }

    // Get the u16 value starting at $(addr), little endian.
    fn parse_u16(&self, addr: u16) -> u16 {
        util::join_u8((self.mem.get(addr), self.mem.get(addr+1)))
    }

    // Push addr to stack
    fn push(&mut self, addr: u16) {
        let split_addr = util::split_u16(addr);
        self.regs.sp -= 2;
        self.mem.set(split_addr.0, self.regs.sp);
        self.mem.set(split_addr.1, self.regs.sp+1);
    }

    // Pop topmost u16 value from stack
    fn pop(&mut self) -> u16 {
        let stack_val = self.parse_u16(self.regs.sp);
        self.regs.sp += 2;
        stack_val
    }

    // Push next_addr to stack, and jump to the jump_addr
    fn call(&mut self, jump_addr: u16) {
        let next_addr = self.regs.pc;
        self.push(next_addr);
        self.regs.pc = jump_addr;
        self.jumped = true;
    }

    // Pop the topmost address from the stack, and jump to it.
    fn ret(&mut self) {
        self.regs.pc = self.pop();
        self.jumped = true;
    }

    // Jump relative to current PC, where offset is twos-complement 8-bit signed int.
    fn jump_relative(&mut self, offset: u8) {
        let addr = self.regs.pc as i32;
        let addr = addr + (offset as i8) as i32;
        if addr < 0 || addr > 0xFFFF {
            println!("Fatal error: jumped out-of-bounds!");
            self.quit = true;
            return;
        }

        self.regs.pc = addr as u16;
        self.jumped = true;
    }

    // Perform given ALU instruction with the given argument
    fn alu(&mut self, op: AluOp, val: u8) {
        let a = self.regs.get_a();
        let cy = if self.regs.get_cy() { 1 } else { 0 };
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
        let old_pc = self.regs.pc;
        let opcode = self.mem.get(self.regs.pc);
        let _operand8  = self.mem.get(self.regs.pc+1);
        let _operand16 = self.parse_u16(self.regs.pc+1);
        self.jumped = false;

        // Adjust opcode if it's a 0xCB prefixed instruction
        let opcode = if opcode == 0xCB {
            self.regs.pc += 1;
            let newop = 0xCB as u16 | _operand8 as u16;
            let _operand8  = self.mem.get(self.regs.pc+1);
            let _operand16 = self.parse_u16(self.regs.pc+1);
            newop
        } else {
            opcode as u16
        };

        let inst = lookup::get_instruction(opcode);

        // Increment PC before we process the instruction. During execution the current PC will
        // represent the next instruction to process.
        self.regs.pc += inst.bytes as u16;

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
                self.regs.sp = _operand16;
            },
            0x32 => {
                let addr = self.regs.get_hl();
                let r = self.regs.get_a();
                self.mem.set(r, addr);
                self.regs.set_hl(addr - 1);
            },
            0x33 => {
                self.regs.sp += 1;
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
                self.regs.sp -= 1;
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
                self.regs.pc = _operand16;
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
