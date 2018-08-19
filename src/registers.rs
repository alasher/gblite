use util;

pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

pub enum Flag {
    Z,
    N,
    H,
    CY
}

pub trait RegOps<R, T> {
    fn get(&self, src: R) -> T;
    fn set(&mut self, dst: R, src: T);
    fn copy(&mut self, dst: R, src: R);
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

pub struct RegisterCache {
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

    // TODO: Remove these, in favor of new RegOps
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

    // TODO: Remove these, in favor of new RegOps
    pub fn get_af(&self) -> u16 { self.af.get_double() }
    pub fn get_bc(&self) -> u16 { self.bc.get_double() }
    pub fn get_de(&self) -> u16 { self.de.get_double() }
    pub fn get_hl(&self) -> u16 { self.hl.get_double() }
    pub fn set_af(&mut self, val: u16) { self.bc.set_double(val); }
    pub fn set_bc(&mut self, val: u16) { self.bc.set_double(val); }
    pub fn set_de(&mut self, val: u16) { self.de.set_double(val); }
    pub fn set_hl(&mut self, val: u16) { self.hl.set_double(val); }

    // TODO: Implement RegOps for Flags
}

impl RegOps<Reg8, u8> for RegisterCache {
    fn get(&self, src: Reg8) -> u8 {
        match src {
            Reg8::A => self.af.get_first(),
            Reg8::B => self.bc.get_first(),
            Reg8::C => self.bc.get_second(),
            Reg8::D => self.de.get_first(),
            Reg8::E => self.de.get_second(),
            Reg8::H => self.hl.get_first(),
            Reg8::L => self.hl.get_second(),
        }
    }

    fn set(&mut self, dst: Reg8, src: u8) {
        match dst {
            Reg8::A => self.af.set_first(src),
            Reg8::B => self.bc.set_first(src),
            Reg8::C => self.bc.set_second(src),
            Reg8::D => self.de.set_first(src),
            Reg8::E => self.de.set_second(src),
            Reg8::H => self.hl.set_first(src),
            Reg8::L => self.hl.set_second(src),
        }
    }

    fn copy(&mut self, dst: Reg8, src: Reg8) {
       let val = self.get(src);
       self.set(dst, val);
    }
}

impl RegOps<Reg16, u16> for RegisterCache {
    fn get(&self, src: Reg16) -> u16 {
        match src {
            Reg16::AF => self.af.get_double(),
            Reg16::BC => self.bc.get_double(),
            Reg16::DE => self.de.get_double(),
            Reg16::HL => self.hl.get_double(),
            Reg16::SP => self.sp,
            Reg16::PC => self.pc
        }
    }

    fn set(&mut self, dst: Reg16, src: u16) {
        match dst {
            Reg16::AF => { self.af.set_double(src) },
            Reg16::BC => { self.bc.set_double(src) },
            Reg16::DE => { self.de.set_double(src) },
            Reg16::HL => { self.hl.set_double(src) },
            Reg16::SP => { self.sp = src; },
            Reg16::PC => { self.pc = src; }
        }
    }

    fn copy(&mut self, dst: Reg16, src: Reg16) {
        let val = self.get(src);
        self.set(dst, val);
    }
}
