use memory::Memory;
use util;

// struct Flags {
//     zero: bool,
//     was_sub: bool,
//     half_carry: bool,
//     carry: bool
// }

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

struct Registers {
    af: DoubleRegister,
    bc: DoubleRegister,
    de: DoubleRegister,
    hl: DoubleRegister,
    sp: u16,
    pc: u16
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
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
    regs: Registers,
    mem: Memory,
    ir_enabled: bool,
    quit: bool,
    jumped: bool
}

impl CPU {
    pub fn new(mem: Memory) -> CPU {
        CPU {
            regs: Registers::new(),
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
    fn call(&mut self, jump_addr: u16, next_addr: u16) {
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
        let addr = addr + (offset as i8) as i32 + 2;
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
            AluOp::Comp     => !val,
            _ => {
                println!("Fatal error: received invalid ALU operation!");
                self.quit = true;
                a
            }
        };

        self.regs.set_a(result);

        // TODO: Add flag mods here
    }

    // Run the instruction at the current PC, return true if successful.
    pub fn process(&mut self) -> bool {
        if self.quit { return false; }
        let mut opname: String = String::from("UNDEFINED");
        let old_pc = self.regs.pc;
        self.jumped = false;

        // TODO: Move these to a separate lookup buffer? It would clean up the
        //       giant match statement below a little bit.
        let mut cycles: u8 = 4;
        let mut opsize: u8 = 1;

        let opcode = self.mem.get(self.regs.pc);
        let operand8  = self.mem.get(self.regs.pc+1);
        let operand16 = self.parse_u16(self.regs.pc+1);

        // Adjust opcode if it's a 0xCB prefixed instruction
        let opcode = if opcode == 0xCB {
            self.regs.pc += 1;
            let newop = (opcode << 2) + operand8;
            let operand8  = self.mem.get(self.regs.pc+1);
            let operand16 = self.mem.get(self.regs.pc+2);
            newop
        } else {
            opcode
        };

        match opcode {
            0x00 => {
                opname = String::from("NOP");
            },
            0x01 => {
                opname = String::from("LD BC,d16");
                self.regs.set_bc(operand16);
                cycles = 12;
                opsize = 3;
            },
            0x02 => {
                opname = String::from("LD (BC),A");
                let r = self.regs.get_a();
                self.mem.set(r, self.regs.get_bc());
                cycles = 8;
            },
            0x03 => {
                opname = String::from("INC BC");
                let val = self.regs.get_bc() + 1;
                self.regs.set_bc(val);
                cycles = 8;
            },
            0x04 => {
                opname = String::from("INC B");
                let val = self.regs.get_b();
                self.regs.set_b(val + 1);
                cycles = 4;
            },
            0x05 => {
                opname = String::from("DEC B");
                let val = self.regs.get_b();
                self.regs.set_b(val - 1);
                cycles = 4;
            },
            0x06 => {
                opname = String::from("LD B,d8");
                self.regs.set_b(operand8);
                cycles = 8;
                opsize = 2;
            },
            0x0A => {
                opname = String::from("LD A,(BC)");
                let r = self.mem.get(self.regs.get_bc());
                self.regs.set_a(r);
                cycles = 8;
            },
            0x0B => {
                opname = String::from("DEC BC");
                let val = self.regs.get_bc() - 1;
                self.regs.set_bc(val);
                cycles = 8;
            },
            0x0C => {
                opname = String::from("INC C");
                let val = self.regs.get_c();
                self.regs.set_c(val + 1);
                cycles = 4;
            },
            0x0D => {
                opname = String::from("DEC C");
                let val = self.regs.get_c();
                self.regs.set_c(val - 1);
                cycles = 4;
            },
            0x0E => {
                opname = String::from("LD C,d8");
                self.regs.set_c(operand8);
                cycles = 8;
                opsize = 2;
            },
            0x10 => {
                opname = String::from("STOP");
                cycles = 4;
                println!("Received STOP instruction, terminating.");
                self.quit = true;
            },
            0x11 => {
                opname = String::from("LD DE,d16");
                self.regs.set_de(operand16);
                cycles = 12;
                opsize = 3;
            },
            0x12 => {
                opname = String::from("LD (DE),A");
                let r = self.regs.get_a();
                self.mem.set(r, self.regs.get_de());
                cycles = 8;
            },
            0x13 => {
                opname = String::from("INC DE");
                let val = self.regs.get_de() + 1;
                self.regs.set_de(val);
                cycles = 8;
            },
            0x14 => {
                opname = String::from("INC D");
                let val = self.regs.get_d();
                self.regs.set_d(val + 1);
                cycles = 4;
            },
            0x15 => {
                opname = String::from("DEC D");
                let val = self.regs.get_d();
                self.regs.set_d(val - 1);
                cycles = 4;
            },
            0x16 => {
                opname = String::from("LD D,d8");
                self.regs.set_d(operand8);
                cycles = 8;
                opsize = 2;
            },
            0x18 => {
                opname = String::from("JR r8");
                self.jump_relative(operand8);
                cycles = 12;
                opsize = 2;
            },
            0x1B => {
                opname = String::from("DEC DE");
                let val = self.regs.get_de() - 1;
                self.regs.set_de(val);
                cycles = 8;
            },
            0x1A => {
                opname = String::from("LD A,(DE)");
                let r = self.mem.get(self.regs.get_de());
                self.regs.set_a(r);
                cycles = 8;
            },
            0x1C => {
                opname = String::from("INC E");
                let val = self.regs.get_e();
                self.regs.set_e(val + 1);
                cycles = 4;
            },
            0x1D => {
                opname = String::from("DEC E");
                let val = self.regs.get_e();
                self.regs.set_e(val - 1);
                cycles = 4;
            },
            0x1E => {
                opname = String::from("LD E,d8");
                self.regs.set_e(operand8);
                cycles = 8;
                opsize = 2;
            },
            0x21 => {
                opname = String::from("LD HL,d16");
                self.regs.set_hl(operand16);
                cycles = 12;
                opsize = 3;
            },
            0x22 => {
                opname = String::from("LD (HL+),A");
                let addr = self.regs.get_hl();
                let r = self.regs.get_a();
                self.mem.set(r, addr);
                self.regs.set_hl(addr + 1);
                cycles = 8;
            },
            0x23 => {
                opname = String::from("INC HL");
                let val = self.regs.get_hl() + 1;
                self.regs.set_hl(val);
                cycles = 8;
            },
            0x24 => {
                opname = String::from("INC H");
                let val = self.regs.get_h();
                self.regs.set_h(val + 1);
                cycles = 4;
            },
            0x25 => {
                opname = String::from("DEC H");
                let val = self.regs.get_h();
                self.regs.set_h(val - 1);
                cycles = 4;
            },
            0x26 => {
                opname = String::from("LD H,d8");
                self.regs.set_h(operand8);
                cycles = 8;
                opsize = 2;
            },
            0x2A => {
                opname = String::from("LD A,(HL+)");
                let addr = self.regs.get_hl();
                let r = self.mem.get(addr);
                self.regs.set_a(r);
                self.regs.set_hl(addr + 1);
                cycles = 8;
            },
            0x2B => {
                opname = String::from("DEC HL");
                let val = self.regs.get_hl() - 1;
                self.regs.set_hl(val);
                cycles = 8;
            },
            0x2C => {
                opname = String::from("INC L");
                let val = self.regs.get_l();
                self.regs.set_l(val + 1);
                cycles = 4;
            },
            0x2D => {
                opname = String::from("DEC L");
                let val = self.regs.get_l();
                self.regs.set_l(val - 1);
                cycles = 4;
            },
            0x2E => {
                opname = String::from("LD L,d8");
                self.regs.set_l(operand8);
                cycles = 8;
                opsize = 2;
            },
            0x31 => {
                opname = String::from("LD SP,d16");
                self.regs.sp = operand16;
                cycles = 12;
                opsize = 3;
            },
            0x32 => {
                opname = String::from("LD (HL-),A");
                let addr = self.regs.get_hl();
                let r = self.regs.get_a();
                self.mem.set(r, addr);
                self.regs.set_hl(addr - 1);
                cycles = 8;
            },
            0x33 => {
                opname = String::from("INC SP");
                self.regs.sp += 1;
                cycles = 8;
            },
            0x34 => {
                opname = String::from("INC (HL)");
                let addr = self.regs.get_hl();
                let val = self.mem.get(addr);
                self.mem.set(val + 1, addr);
                cycles = 4;
            },
            0x35 => {
                opname = String::from("DEC (HL)");
                let addr = self.regs.get_hl();
                let val = self.mem.get(addr);
                self.mem.set(val - 1, addr);
                cycles = 4;
            },
            0x36 => {
                opname = String::from("LD (hl),d8");
                self.mem.set(operand8, self.regs.get_hl());
                cycles = 12;
                opsize = 2;
            },
            0x3A => {
                opname = String::from("LD A,(HL-)");
                let addr = self.regs.get_hl();
                let r = self.mem.get(addr);
                self.regs.set_a(r);
                self.regs.set_hl(addr - 1);
                cycles = 8;
            },
            0x3B => {
                opname = String::from("DEC SP");
                self.regs.sp -= 1;
                cycles = 8;
            },
            0x3C => {
                opname = String::from("INC A");
                let val = self.regs.get_a();
                self.regs.set_a(val + 1);
                cycles = 4;
            },
            0x3D => {
                opname = String::from("DEC A");
                let val = self.regs.get_a();
                self.regs.set_a(val - 1);
                cycles = 4;
            },
            0x3E => {
                opname = String::from("LD A,d8");
                self.regs.set_a(operand8);
                cycles = 8;
                opsize = 2;
            },
            0x40 => {
                opname = String::from("LD B,B");
                let r = self.regs.get_b();
                self.regs.set_b(r);
            },
            0x41 => {
                opname = String::from("LD B,C");
                let r = self.regs.get_c();
                self.regs.set_b(r);
            },
            0x42 => {
                opname = String::from("LD B,D");
                let r = self.regs.get_d();
                self.regs.set_b(r);
            },
            0x43 => {
                opname = String::from("LD B,E");
                let r = self.regs.get_e();
                self.regs.set_b(r);
            },
            0x44 => {
                opname = String::from("LD B,H");
                let r = self.regs.get_h();
                self.regs.set_b(r);
            },
            0x45 => {
                opname = String::from("LD B,L");
                let r = self.regs.get_l();
                self.regs.set_b(r);
            },
            0x46 => {
                opname = String::from("LD B,(HL)");
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_b(r);
                cycles = 8;
            },
            0x47 => {
                opname = String::from("LD B,A");
                let r = self.regs.get_a();
                self.regs.set_b(r);
            },
            0x48 => {
                opname = String::from("LD C,B");
                let r = self.regs.get_b();
                self.regs.set_c(r);
            },
            0x49 => {
                opname = String::from("LD C,C");
                let r = self.regs.get_c();
                self.regs.set_c(r);
            },
            0x4a => {
                opname = String::from("LD C,D");
                let r = self.regs.get_d();
                self.regs.set_c(r);
            },
            0x4b => {
                opname = String::from("LD C,E");
                let r = self.regs.get_e();
                self.regs.set_c(r);
            },
            0x4c => {
                opname = String::from("LD C,H");
                let r = self.regs.get_h();
                self.regs.set_c(r);
            },
            0x4d => {
                opname = String::from("LD C,L");
                let r = self.regs.get_l();
                self.regs.set_c(r);
            },
            0x4e => {
                opname = String::from("LD C,(HL)");
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_c(r);
                cycles = 8;
            },
            0x4f => {
                opname = String::from("LD C,A");
                let r = self.regs.get_a();
                self.regs.set_c(r);
            },
            0x50 => {
                opname = String::from("LD D,B");
                let r = self.regs.get_b();
                self.regs.set_d(r);
            },
            0x51 => {
                opname = String::from("LD D,C");
                let r = self.regs.get_c();
                self.regs.set_d(r);
            },
            0x52 => {
                opname = String::from("LD D,D");
                let r = self.regs.get_d();
                self.regs.set_d(r);
            },
            0x53 => {
                opname = String::from("LD D,E");
                let r = self.regs.get_e();
                self.regs.set_d(r);
            },
            0x54 => {
                opname = String::from("LD D,H");
                let r = self.regs.get_h();
                self.regs.set_d(r);
            },
            0x55 => {
                opname = String::from("LD D,L");
                let r = self.regs.get_l();
                self.regs.set_d(r);
            },
            0x56 => {
                opname = String::from("LD D,(HL)");
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_d(r);
                cycles = 8;
            },
            0x57 => {
                opname = String::from("LD D,A");
                let r = self.regs.get_a();
                self.regs.set_d(r);
            },
            0x58 => {
                opname = String::from("LD E,B");
                let r = self.regs.get_b();
                self.regs.set_e(r);
            },
            0x59 => {
                opname = String::from("LD E,C");
                let r = self.regs.get_c();
                self.regs.set_e(r);
            },
            0x5a => {
                opname = String::from("LD E,D");
                let r = self.regs.get_d();
                self.regs.set_e(r);
            },
            0x5b => {
                opname = String::from("LD E,E");
                let r = self.regs.get_e();
                self.regs.set_e(r);
            },
            0x5c => {
                opname = String::from("LD E,H");
                let r = self.regs.get_h();
                self.regs.set_e(r);
            },
            0x5d => {
                opname = String::from("LD E,L");
                let r = self.regs.get_l();
                self.regs.set_e(r);
            },
            0x5e => {
                opname = String::from("LD E,(HL)");
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_e(r);
                cycles = 8;
            },
            0x5f => {
                opname = String::from("LD E,A");
                let r = self.regs.get_a();
                self.regs.set_e(r);
            },
            0x60 => {
                opname = String::from("LD H,B");
                let r = self.regs.get_b();
                self.regs.set_h(r);
            },
            0x61 => {
                opname = String::from("LD H,C");
                let r = self.regs.get_c();
                self.regs.set_h(r);
            },
            0x62 => {
                opname = String::from("LD H,D");
                let r = self.regs.get_d();
                self.regs.set_h(r);
            },
            0x63 => {
                opname = String::from("LD H,E");
                let r = self.regs.get_e();
                self.regs.set_h(r);
            },
            0x64 => {
                opname = String::from("LD H,H");
                let r = self.regs.get_h();
                self.regs.set_h(r);
            },
            0x65 => {
                opname = String::from("LD H,L");
                let r = self.regs.get_l();
                self.regs.set_h(r);
            },
            0x66 => {
                opname = String::from("LD H,(HL)");
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_h(r);
                cycles = 8;
            },
            0x67 => {
                opname = String::from("LD H,A");
                let r = self.regs.get_a();
                self.regs.set_h(r);
            },
            0x68 => {
                opname = String::from("LD L,B");
                let r = self.regs.get_b();
                self.regs.set_l(r);
            },
            0x69 => {
                opname = String::from("LD L,C");
                let r = self.regs.get_c();
                self.regs.set_l(r);
            },
            0x6a => {
                opname = String::from("LD L,D");
                let r = self.regs.get_d();
                self.regs.set_l(r);
            },
            0x6b => {
                opname = String::from("LD L,E");
                let r = self.regs.get_e();
                self.regs.set_l(r);
            },
            0x6c => {
                opname = String::from("LD L,H");
                let r = self.regs.get_h();
                self.regs.set_l(r);
            },
            0x6d => {
                opname = String::from("LD L,L");
                let r = self.regs.get_l();
                self.regs.set_l(r);
            },
            0x6e => {
                opname = String::from("LD L,(HL)");
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_l(r);
                cycles = 8;
            },
            0x6f => {
                opname = String::from("LD L,A");
                let r = self.regs.get_a();
                self.regs.set_l(r);
            },
            0x70 => {
                opname = String::from("LD (HL),B");
                let r = self.regs.get_b();
                self.mem.set(r, self.regs.get_hl());
                cycles = 8;
            },
            0x71 => {
                opname = String::from("LD (HL),C");
                let r = self.regs.get_c();
                self.mem.set(r, self.regs.get_hl());
                cycles = 8;
            },
            0x72 => {
                opname = String::from("LD (HL),D");
                let r = self.regs.get_d();
                self.mem.set(r, self.regs.get_hl());
                cycles = 8;
            },
            0x73 => {
                opname = String::from("LD (HL),E");
                let r = self.regs.get_e();
                self.mem.set(r, self.regs.get_hl());
                cycles = 8;
            },
            0x74 => {
                opname = String::from("LD (HL),H");
                let r = self.regs.get_h();
                self.mem.set(r, self.regs.get_hl());
                cycles = 8;
            },
            0x75 => {
                opname = String::from("LD (HL),L");
                let r = self.regs.get_l();
                self.mem.set(r, self.regs.get_hl());
                cycles = 8;
            },
            0x76 => {
                println!("Encountered HALT instruction, exiting!");
                opname = String::from("HALT");
                self.quit = true;
            },
            0x77 => {
                opname = String::from("LD (HL),A");
                let r = self.regs.get_a();
                self.mem.set(r, self.regs.get_hl());
                cycles = 8;
            },
            0x78 => {
                opname = String::from("LD A,B");
                let r = self.regs.get_b();
                self.regs.set_a(r);
            },
            0x79 => {
                opname = String::from("LD A,C");
                let r = self.regs.get_c();
                self.regs.set_a(r);
            },
            0x7a => {
                opname = String::from("LD A,D");
                let r = self.regs.get_d();
                self.regs.set_a(r);
            },
            0x7b => {
                opname = String::from("LD A,E");
                let r = self.regs.get_e();
                self.regs.set_a(r);
            },
            0x7c => {
                opname = String::from("LD A,H");
                let r = self.regs.get_h();
                self.regs.set_a(r);
            },
            0x7d => {
                opname = String::from("LD A,L");
                let r = self.regs.get_l();
                self.regs.set_a(r);
            },
            0x7e => {
                opname = String::from("LD A,(HL)");
                let r = self.mem.get(self.regs.get_hl());
                self.regs.set_a(r);
                cycles = 8;
            },
            0x7f => {
                opname = String::from("LD A,A");
                let r = self.regs.get_a();
                self.regs.set_a(r);
            },
            0xC1 => {
                opname = String::from("POP BC");
                let val = self.pop();
                self.regs.set_bc(val);
                cycles = 12;
            },
            0xC3 => {
                opname = String::from("JP a16");
                self.regs.pc = operand16;
                self.jumped = true;
                cycles = 16;
                opsize = 3;
            },
            0xC5 => {
                opname = String::from("PUSH BC");
                let reg = self.regs.get_bc();
                self.push(reg);
                cycles = 16;
            },
            0xC7 => {
                opname = String::from("RST 00H");
                let next_addr = self.regs.pc + 1;
                self.call(0x00, next_addr);
                cycles = 16;
            },
            0xC9 => {
                opname = String::from("RET");
                self.ret();
                cycles = 16;
            },
            0xCB => {
                // This should never happen, we should always append the prefix after CB, ex: 0xCB01
                println!("Fatal error: encountered unadjusted 0xCB literal!");
                opname = String::from("PREFIX CB");
                self.quit = true;
            },
            0xCD => {
                opname = String::from("CALL a16");
                opsize = 3;
                let next_addr = self.regs.pc + opsize as u16;
                self.call(operand16, next_addr);
                cycles = 24;
            },
            0xCF => {
                opname = String::from("RST 08H");
                let next_addr = self.regs.pc + 1;
                self.call(0x08, next_addr);
                cycles = 16;
            },
            0xD1 => {
                opname = String::from("POP DE");
                let val = self.pop();
                self.regs.set_de(val);
                cycles = 12;
            },
            0xD5 => {
                opname = String::from("PUSH DE");
                let reg = self.regs.get_de();
                self.push(reg);
                cycles = 16;
            },
            0xD7 => {
                opname = String::from("RST 10H");
                let next_addr = self.regs.pc + 1;
                self.call(0x10, next_addr);
                cycles = 16;
            },
            0xD9 => {
                opname = String::from("RETI");
                self.ret();
                self.ir_enabled = true;
                cycles = 16;
            },
            0xDF => {
                opname = String::from("RST 18H");
                let next_addr = self.regs.pc + 1;
                self.call(0x18, next_addr);
                cycles = 16;
            },
            0xE0 => {
                opname = String::from("LDH (a8),A");
                self.mem.set(self.regs.get_a(), 0xFF00 + (operand8 as u16));
                cycles = 12;
                opsize = 2;
            },
            0xE1 => {
                opname = String::from("POP HL");
                let val = self.pop();
                self.regs.set_hl(val);
                cycles = 12;
            },
            0xE2 => {
                opname = String::from("LD (C),A");
                let addr = 0xFF00 + self.regs.get_c() as u16;
                self.mem.set(self.regs.get_a(), addr);
                cycles = 8;
                opsize = 2;
            },
            0xE7 => {
                opname = String::from("RST 20H");
                let next_addr = self.regs.pc + 1;
                self.call(0x20, next_addr);
                cycles = 16;
            },
            0xEA => {
                opname = String::from("LD (a16),A");
                self.mem.set(self.regs.get_a(), operand16);
                cycles = 16;
                opsize = 3;
            },
            0xE5 => {
                opname = String::from("PUSH HL");
                let reg = self.regs.get_hl();
                self.push(reg);
                cycles = 16;
            },
            0xEF => {
                opname = String::from("RST 28H");
                let next_addr = self.regs.pc + 1;
                self.call(0x28, next_addr);
                cycles = 16;
            },
            0xF0 => {
                opname = String::from("LDH A,(a8)");
                self.regs.set_a(self.mem.get(0xFF00 + (operand8 as u16)));
                cycles = 12;
                opsize = 2;
            },
            0xF1 => {
                opname = String::from("POP AF");
                let val = self.pop();
                self.regs.set_af(val);
                cycles = 12;
            },
            0xF2 => {
                opname = String::from("LD (C),A");
                let addr = 0xFF00 + self.regs.get_c() as u16;
                self.mem.set(self.regs.get_a(), addr);
                cycles = 8;
                opsize = 2;
            },
            0xF3 => {
                opname = String::from("DI");
                self.ir_enabled = false;
            },
            0xFA => {
                opname = String::from("LD A,(a16)");
                self.regs.set_a(self.mem.get(operand16));
                cycles = 16;
                opsize = 3;
            },
            0xFB => {
                opname = String::from("EI");
                self.ir_enabled = true;
            },
            0xF5 => {
                opname = String::from("PUSH AF");
                let reg = self.regs.get_af();
                self.push(reg);
                cycles = 16;
            },
            0xF7 => {
                opname = String::from("RST 30H");
                let next_addr = self.regs.pc + 1;
                self.call(0x30, next_addr);
                cycles = 16;
            },
            0xFF => {
                opname = String::from("RST 38H");
                let next_addr = self.regs.pc + 1;
                self.call(0x38, next_addr);
                cycles = 16;
            },
            _ => {
                println!("Fatal error: undefined instruction!");
                opname = format!("UNDEFINED 0x{:02X}", opcode);
                self.quit = true;
            }
        }

        self.print_instruction_debug(opname.as_str(), cycles, old_pc, opsize);

        // Standard PC increment, a single instruction
        if !self.jumped {
            self.regs.pc += opsize as u16;
        }


        !self.quit
    }

    fn print_instruction_debug(&self, opname: &str, ncycles: u8, old_pc: u16, opsize: u8) {
        let mut pstr = format!("0x{:04x}: {} - {} cycles", old_pc, opname, ncycles);
        if opsize > 1 {
            pstr += " - operands: ";
            for i in 1..opsize {
                pstr += &format!("0x{:02x} ", self.mem.get(old_pc + i as u16));
            }
        }
        println!("{}", pstr);
    }
}
