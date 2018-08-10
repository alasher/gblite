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
            pc: 0x100 // Skip the Boot ROM
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

    fn parse_u16(&self, iaddr: usize) -> u16 {
        let addr = (self.mem.get(iaddr+1) as u16) << 8;
        let addr = addr + self.mem.get(iaddr) as u16;
        addr
    }

    // Run the instruction at the current PC
    pub fn process(&mut self) -> bool {
        let iaddr = usize::from(self.regs.pc);
        let mut jump: bool = false;
        let mut quit: bool = false;
        let mut opname: String = String::from("UNDEFINED");

        let opcode = self.mem.get(iaddr);
        let opcode = if opcode == 0xCB {
            self.regs.pc += 1;
            (opcode << 2) + self.mem.get(iaddr+1)
        } else {
            opcode
        };

        // TODO: Finish adding instructions below
        match opcode {
            0x00 => {
                opname = String::from("NOP");
            },
            0xC3 => {
                let addr = self.parse_u16(iaddr+1);
                opname = format!("JP a16 (0x{:04x})", addr);
                self.regs.pc = addr;
                jump = true;
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
                opname = format!("UNDEFINED {:2x}", opcode);
                quit = true;
            }
        }

        if !jump {
            self.regs.pc += 1;
        }

        println!("0x{:04x}: {}", iaddr, opname);

        !quit
    }
}
