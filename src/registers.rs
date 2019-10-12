use std::marker::Copy;
use std::clone::Clone;

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::cmp::PartialEq;
use std::fmt::{Display, LowerHex};
use num::Num;
use num::FromPrimitive;
use num::traits::{WrappingAdd, WrappingSub};


#[derive(Copy, Clone)]
pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L
}

#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

pub trait Reg : Clone + Copy {}
impl Reg for Reg8  {}
impl Reg for Reg16 {}

pub trait RegData<T> : Num + Clone + Copy + PartialEq + BitAnd<Output=T> + BitOr<Output=T>
     + BitXor<Output=T> + Not<Output=T> + WrappingAdd + WrappingSub + PartialOrd + FromPrimitive + Display + LowerHex {}
impl<T> RegData<T> for T where T: Num + Clone + Copy + PartialEq + BitAnd<Output=T> + BitOr<Output=T>
     + BitXor<Output=T> + Not<Output=T> + WrappingAdd + WrappingSub + PartialOrd + FromPrimitive + Display + LowerHex {}

#[derive(Copy, Clone)]
pub enum Flag {
    Z,
    N,
    H,
    CY
}

pub enum FlagMod {
    Ignore,
    Eval,
    Set(bool)
}

pub struct FlagStatus {
    pub z:  FlagMod, // Flag modifiers: for each flag, define if this instruction ignores this
    pub n:  FlagMod, // flag, sets this flag to a fixed value, or sets it to a value that is
    pub h:  FlagMod, // evaluated within this instruction.
    pub cy: FlagMod
}

pub trait RegOps<R: Reg, T: RegData<T>> {
    fn get(&self, src: R) -> T;
    fn set(&mut self, dst: R, src: T);
    fn copy(&mut self, dst: R, src: R) {
       let tmp = self.get(src);
       self.set(dst, tmp);
    }
    fn add(&mut self, dst: R, val: T) {
        let new_val = self.get(dst).wrapping_add(&val);
        self.set(dst, new_val);
    }
    fn sub(&mut self, dst: R, val: T) {
        let new_val = self.get(dst).wrapping_sub(&val);
        self.set(dst, new_val);
    }
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

    pub fn print_contents(&self) {
        println!("(0x{:02x}, 0x{:02x}, 0x{:04x})", self.get_first(), self.get_second(), self.get_double());
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

    fn flag_mask(f: Flag) -> u8 {
        1 << (match f {
            Flag::Z  => 7,
            Flag::N  => 6,
            Flag::H  => 5,
            Flag::CY => 4
        })
    }

    pub fn get_flag(&self, f: Flag) -> bool {
        let mask = RegisterCache::flag_mask(f);
        (self.get(Reg8::F) & mask) != 0
    }

    pub fn set_flag(&mut self, f: Flag, val: bool) {
        let flags = self.get(Reg8::F);
        let mask = RegisterCache::flag_mask(f);

        let flags = if val { 
            flags | mask
        } else {
            flags & !mask
        };

        self.set(Reg8::F, flags);
    }

    pub fn print_registers(&self) {
        print!("AF: ");
        self.af.print_contents();
        print!("BC: ");
        self.bc.print_contents();
        print!("DE: ");
        self.de.print_contents();
        print!("HL: ");
        self.hl.print_contents();
        println!("Flags: Z: {}, N: {}, H: {}, CY: {}",
                 self.get_flag(Flag::Z),
                 self.get_flag(Flag::N),
                 self.get_flag(Flag::H),
                 self.get_flag(Flag::CY));
        println!("PC: 0x{:04x}, SP: 0x{:04x}", self.pc, self.sp);
    }
}

impl RegOps<Reg8, u8> for RegisterCache {
    fn get(&self, src: Reg8) -> u8 {
        match src {
            Reg8::A => self.af.get_first(),
            Reg8::F => self.af.get_second(),
            Reg8::B => self.bc.get_first(),
            Reg8::C => self.bc.get_second(),
            Reg8::D => self.de.get_first(),
            Reg8::E => self.de.get_second(),
            Reg8::H => self.hl.get_first(),
            Reg8::L => self.hl.get_second()
        }
    }

    fn set(&mut self, dst: Reg8, src: u8) {
        match dst {
            Reg8::A => self.af.set_first(src),
            Reg8::F => self.af.set_second(src),
            Reg8::B => self.bc.set_first(src),
            Reg8::C => self.bc.set_second(src),
            Reg8::D => self.de.set_first(src),
            Reg8::E => self.de.set_second(src),
            Reg8::H => self.hl.set_first(src),
            Reg8::L => self.hl.set_second(src)
        }
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
}
