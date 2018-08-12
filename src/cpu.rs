use memory::Memory;

struct Flags {
    zero: bool,
    was_sub: bool,
    half_carry: bool,
    carry: bool
}

struct DoubleRegister {
    lo: u8,
    hi: u8
}

impl DoubleRegister {
    fn get_lo(&self) -> u8 { self.lo }
    fn set_lo(&mut self, val: u8) { self.lo = val; }
    fn get_hi(&self) -> u8 { self.hi }
    fn set_hi(&mut self, val: u8) { self.hi = val; }

    fn get_double(&self) -> u16 {
        (u16::from(self.get_hi()) << 8) | u16::from(self.get_lo())
    }
    fn set_double(&mut self, val: u16) {
        self.lo = (val & 0xFF) as u8;
        self.hi = (val >> 8)   as u8;
    }
}

struct Registers {
    a:  u8,
    f:  Flags,
    bc: DoubleRegister,
    de: DoubleRegister,
    hl: DoubleRegister,
    sp: u16,
    pc: u16
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x0,
            f: Flags{zero: false, was_sub: false, half_carry: false, carry: false},
            bc: DoubleRegister{lo: 0x0, hi: 0x0},
            de: DoubleRegister{lo: 0x0, hi: 0x0},
            hl: DoubleRegister{lo: 0x0, hi: 0x0},
            sp: 0x0,
            pc: 0x0
        }
    }

    // Get/Set for 8-bit registers
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.bc.get_lo() }
    pub fn get_c(&self) -> u8 { self.bc.get_hi() }
    pub fn get_d(&self) -> u8 { self.de.get_lo() }
    pub fn get_e(&self) -> u8 { self.de.get_hi() }
    pub fn get_h(&self) -> u8 { self.hl.get_lo() }
    pub fn get_l(&self) -> u8 { self.hl.get_hi() }
    pub fn set_a(&mut self, val: u8) { self.a = val; }
    pub fn set_b(&mut self, val: u8) { self.bc.set_lo(val); }
    pub fn set_c(&mut self, val: u8) { self.bc.set_hi(val); }
    pub fn set_d(&mut self, val: u8) { self.de.set_lo(val); }
    pub fn set_e(&mut self, val: u8) { self.de.set_hi(val); }
    pub fn set_h(&mut self, val: u8) { self.hl.set_lo(val); }
    pub fn set_l(&mut self, val: u8) { self.hl.set_hi(val); }

    // Get/Set for 16-bit registers
    pub fn get_sp(&self) -> u16 { self.sp }
    pub fn get_pc(&self) -> u16 { self.pc }
    pub fn get_bc(&self) -> u16 { self.bc.get_double() }
    pub fn get_de(&self) -> u16 { self.de.get_double() }
    pub fn get_hl(&self) -> u16 { self.hl.get_double() }
    pub fn set_sp(&mut self, val: u16) { self.sp = val; }
    pub fn set_pc(&mut self, val: u16) { self.pc = val; }
    pub fn set_bc(&mut self, val: u16) { self.bc.set_double(val); }
    pub fn set_de(&mut self, val: u16) { self.de.set_double(val); }
    pub fn set_hl(&mut self, val: u16) { self.hl.set_double(val); }
}

pub struct CPU {
    regs: Registers,
    mem: Memory,
    ir_enabled: bool
}

impl CPU {
    pub fn new(mem: Memory) -> CPU {
        CPU {
            regs: Registers::new(),
            mem: mem,
            ir_enabled: true
        }
    }

    fn parse_u16(&self, iaddr: u16) -> u16 {
        ((self.mem.get(iaddr+1) as u16) << 8) | self.mem.get(iaddr) as u16
    }

    // Run the instruction at the current PC, return true if successful.
    pub fn process(&mut self) -> bool {
        let mut quit: bool = false;
        let mut opname: String = String::from("UNDEFINED");
        let mut cycles: u8 = 0;
        let old_pc = self.regs.pc;

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

        // TODO: Finish adding instructions below
        match opcode {
            0x00 => {
                opname = String::from("NOP");
            },
            0x06 => {
                opname = String::from("LD B,d8");
                self.regs.set_b(operand8);
                cycles += 8;
                self.regs.pc += 2;
            },
            0x0E => {
                opname = String::from("LD C,d8");
                self.regs.set_c(operand8);
                cycles += 8;
                self.regs.pc += 2;
            },
            0x16 => {
                opname = String::from("LD D,d8");
                self.regs.set_d(operand8);
                cycles += 8;
                self.regs.pc += 2;
            },
            0x1E => {
                opname = String::from("LD E,d8");
                self.regs.set_e(operand8);
                cycles += 8;
                self.regs.pc += 2;
            },
            0x26 => {
                opname = String::from("LD H,d8");
                self.regs.set_h(operand8);
                cycles += 8;
                self.regs.pc += 2;
            },
            0x2E => {
                opname = String::from("LD L,d8");
                self.regs.set_l(operand8);
                cycles += 8;
                self.regs.pc += 2;
            },
            0x31 => {
                opname = String::from("LD SP,d16");
                self.regs.set_sp(operand16);
                self.regs.pc += 3;
            },
            0x36 => {
                opname = String::from("LD (hl),d8");
                self.mem.set(operand8, self.regs.get_hl());
                cycles += 12;
                self.regs.pc += 2;
            },
            0x3E => {
                opname = String::from("LD A,d8");
                self.regs.set_a(operand8);
                cycles += 8;
                self.regs.pc += 2;
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
                cycles += 8;
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
                cycles += 8;
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
                cycles += 8;
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
                cycles += 8;
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
                cycles += 8;
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
                cycles += 8;
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
                cycles += 8;
            },
            0x71 => {
                opname = String::from("LD (HL),C");
                let r = self.regs.get_c();
                self.mem.set(r, self.regs.get_hl());
                cycles += 8;
            },
            0x72 => {
                opname = String::from("LD (HL),D");
                let r = self.regs.get_d();
                self.mem.set(r, self.regs.get_hl());
                cycles += 8;
            },
            0x73 => {
                opname = String::from("LD (HL),E");
                let r = self.regs.get_e();
                self.mem.set(r, self.regs.get_hl());
                cycles += 8;
            },
            0x74 => {
                opname = String::from("LD (HL),H");
                let r = self.regs.get_h();
                self.mem.set(r, self.regs.get_hl());
                cycles += 8;
            },
            0x75 => {
                opname = String::from("LD (HL),L");
                let r = self.regs.get_l();
                self.mem.set(r, self.regs.get_hl());
                cycles += 8;
            },
            0x76 => {
                println!("Encountered HALT instruction, exiting!");
                opname = String::from("HALT");
                quit = true;
            },
            0x77 => {
                opname = String::from("LD (HL),A");
                let r = self.regs.get_a();
                self.mem.set(r, self.regs.get_hl());
                cycles += 8;
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
                cycles += 8;
            },
            0x7f => {
                opname = String::from("LD A,A");
                let r = self.regs.get_a();
                self.regs.set_a(r);
            },
            0xC3 => {
                opname = format!("JP a16 (0x{:04x})", operand16);
                self.regs.pc = operand16;
                cycles += 16;
            },
            0xCB => {
                // This should never happen, we should always append the prefix after CB, ex: 0xCB01
                println!("Fatal error: encountered unadjusted 0xCB literal!");
                opname = String::from("PREFIX CB");
                quit = true;
            },
            0xF3 => {
                opname = String::from("DI");
                self.ir_enabled = false;
            },
            0xFB => {
                opname = String::from("EI");
                self.ir_enabled = true;
            },
            _ => {
                println!("Fatal error: undefined instruction!");
                opname = format!("UNDEFINED 0x{:02X}", opcode);
                quit = true;
            }
        }

        // Standard cycle count, 4
        if cycles == 0 {
            cycles = 4;
        }

        // Standard PC increment, a single instruction
        if self.regs.pc == old_pc {
            self.regs.pc += 1;
        }

        println!("0x{:04x}: {} - {} cycles", old_pc, opname, cycles);

        !quit
    }
}
