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

pub fn run() {
    println!("Running the CPU!");
}
