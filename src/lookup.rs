#![allow(dead_code)]

use registers::FlagMod;
use registers::FlagStatus;

pub struct Instruction {
    pub opcode: u8,           // The byte opcode of this instruction.
    pub prefix_cb: bool,      // Indicates if this opcode is part of the 0xCB extended instruction set.
    pub name: String,         // The name of this instruction.
    pub bytes: u8,            // The total number of bytes of this instruction, including all byte(s)
                              // required for the opcode.
    pub clocks: u8,           // Minimum number of clocks required.
    pub clocks_extra: u8,     // For conditional instructions, the number of extra clocks to take if the
                              // longer instruction path is taken. Ex: JP, RET, etc.
    pub modifies_flags: bool  // True if any flag could be modified by this instruction
}

pub fn get_instruction(opcode: u16) -> Instruction {
    match opcode {
        0x0 => Instruction {
            opcode: 0x0,
            prefix_cb: false,
            name: String::from("NOP"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x1 => Instruction {
            opcode: 0x1,
            prefix_cb: false,
            name: String::from("LD BC,d16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x2 => Instruction {
            opcode: 0x2,
            prefix_cb: false,
            name: String::from("LD (BC),A"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x3 => Instruction {
            opcode: 0x3,
            prefix_cb: false,
            name: String::from("INC BC"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x4 => Instruction {
            opcode: 0x4,
            prefix_cb: false,
            name: String::from("INC B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x5 => Instruction {
            opcode: 0x5,
            prefix_cb: false,
            name: String::from("DEC B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x6 => Instruction {
            opcode: 0x6,
            prefix_cb: false,
            name: String::from("LD B,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x7 => Instruction {
            opcode: 0x7,
            prefix_cb: false,
            name: String::from("RLCA"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x8 => Instruction {
            opcode: 0x8,
            prefix_cb: false,
            name: String::from("LD (a16),SP"),
            bytes: 3,
            clocks: 20,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x9 => Instruction {
            opcode: 0x9,
            prefix_cb: false,
            name: String::from("ADD HL,BC"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa => Instruction {
            opcode: 0xa,
            prefix_cb: false,
            name: String::from("LD A,(BC)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xb => Instruction {
            opcode: 0xb,
            prefix_cb: false,
            name: String::from("DEC BC"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xc => Instruction {
            opcode: 0xc,
            prefix_cb: false,
            name: String::from("INC C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xd => Instruction {
            opcode: 0xd,
            prefix_cb: false,
            name: String::from("DEC C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xe => Instruction {
            opcode: 0xe,
            prefix_cb: false,
            name: String::from("LD C,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf => Instruction {
            opcode: 0xf,
            prefix_cb: false,
            name: String::from("RRCA"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x10 => Instruction {
            opcode: 0x10,
            prefix_cb: false,
            name: String::from("STOP 0"),
            bytes: 2,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x11 => Instruction {
            opcode: 0x11,
            prefix_cb: false,
            name: String::from("LD DE,d16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x12 => Instruction {
            opcode: 0x12,
            prefix_cb: false,
            name: String::from("LD (DE),A"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x13 => Instruction {
            opcode: 0x13,
            prefix_cb: false,
            name: String::from("INC DE"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x14 => Instruction {
            opcode: 0x14,
            prefix_cb: false,
            name: String::from("INC D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x15 => Instruction {
            opcode: 0x15,
            prefix_cb: false,
            name: String::from("DEC D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x16 => Instruction {
            opcode: 0x16,
            prefix_cb: false,
            name: String::from("LD D,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x17 => Instruction {
            opcode: 0x17,
            prefix_cb: false,
            name: String::from("RLA"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x18 => Instruction {
            opcode: 0x18,
            prefix_cb: false,
            name: String::from("JR r8"),
            bytes: 2,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x19 => Instruction {
            opcode: 0x19,
            prefix_cb: false,
            name: String::from("ADD HL,DE"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x1a => Instruction {
            opcode: 0x1a,
            prefix_cb: false,
            name: String::from("LD A,(DE)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x1b => Instruction {
            opcode: 0x1b,
            prefix_cb: false,
            name: String::from("DEC DE"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x1c => Instruction {
            opcode: 0x1c,
            prefix_cb: false,
            name: String::from("INC E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x1d => Instruction {
            opcode: 0x1d,
            prefix_cb: false,
            name: String::from("DEC E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x1e => Instruction {
            opcode: 0x1e,
            prefix_cb: false,
            name: String::from("LD E,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x1f => Instruction {
            opcode: 0x1f,
            prefix_cb: false,
            name: String::from("RRA"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x20 => Instruction {
            opcode: 0x20,
            prefix_cb: false,
            name: String::from("JR NZ,r8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 4,
            modifies_flags: false
        },
        0x21 => Instruction {
            opcode: 0x21,
            prefix_cb: false,
            name: String::from("LD HL,d16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x22 => Instruction {
            opcode: 0x22,
            prefix_cb: false,
            name: String::from("LD (HL+),A"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x23 => Instruction {
            opcode: 0x23,
            prefix_cb: false,
            name: String::from("INC HL"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x24 => Instruction {
            opcode: 0x24,
            prefix_cb: false,
            name: String::from("INC H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x25 => Instruction {
            opcode: 0x25,
            prefix_cb: false,
            name: String::from("DEC H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x26 => Instruction {
            opcode: 0x26,
            prefix_cb: false,
            name: String::from("LD H,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x27 => Instruction {
            opcode: 0x27,
            prefix_cb: false,
            name: String::from("DAA"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x28 => Instruction {
            opcode: 0x28,
            prefix_cb: false,
            name: String::from("JR Z,r8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 4,
            modifies_flags: false
        },
        0x29 => Instruction {
            opcode: 0x29,
            prefix_cb: false,
            name: String::from("ADD HL,HL"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x2a => Instruction {
            opcode: 0x2a,
            prefix_cb: false,
            name: String::from("LD A,(HL+)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x2b => Instruction {
            opcode: 0x2b,
            prefix_cb: false,
            name: String::from("DEC HL"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x2c => Instruction {
            opcode: 0x2c,
            prefix_cb: false,
            name: String::from("INC L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x2d => Instruction {
            opcode: 0x2d,
            prefix_cb: false,
            name: String::from("DEC L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x2e => Instruction {
            opcode: 0x2e,
            prefix_cb: false,
            name: String::from("LD L,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x2f => Instruction {
            opcode: 0x2f,
            prefix_cb: false,
            name: String::from("CPL"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x30 => Instruction {
            opcode: 0x30,
            prefix_cb: false,
            name: String::from("JR NC,r8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 4,
            modifies_flags: false
        },
        0x31 => Instruction {
            opcode: 0x31,
            prefix_cb: false,
            name: String::from("LD SP,d16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x32 => Instruction {
            opcode: 0x32,
            prefix_cb: false,
            name: String::from("LD (HL-),A"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x33 => Instruction {
            opcode: 0x33,
            prefix_cb: false,
            name: String::from("INC SP"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x34 => Instruction {
            opcode: 0x34,
            prefix_cb: false,
            name: String::from("INC (HL)"),
            bytes: 1,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x35 => Instruction {
            opcode: 0x35,
            prefix_cb: false,
            name: String::from("DEC (HL)"),
            bytes: 1,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x36 => Instruction {
            opcode: 0x36,
            prefix_cb: false,
            name: String::from("LD (HL),d8"),
            bytes: 2,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x37 => Instruction {
            opcode: 0x37,
            prefix_cb: false,
            name: String::from("SCF"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x38 => Instruction {
            opcode: 0x38,
            prefix_cb: false,
            name: String::from("JR C,r8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 4,
            modifies_flags: false
        },
        0x39 => Instruction {
            opcode: 0x39,
            prefix_cb: false,
            name: String::from("ADD HL,SP"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x3a => Instruction {
            opcode: 0x3a,
            prefix_cb: false,
            name: String::from("LD A,(HL-)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x3b => Instruction {
            opcode: 0x3b,
            prefix_cb: false,
            name: String::from("DEC SP"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x3c => Instruction {
            opcode: 0x3c,
            prefix_cb: false,
            name: String::from("INC A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x3d => Instruction {
            opcode: 0x3d,
            prefix_cb: false,
            name: String::from("DEC A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x3e => Instruction {
            opcode: 0x3e,
            prefix_cb: false,
            name: String::from("LD A,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x3f => Instruction {
            opcode: 0x3f,
            prefix_cb: false,
            name: String::from("CCF"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x40 => Instruction {
            opcode: 0x40,
            prefix_cb: false,
            name: String::from("LD B,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x41 => Instruction {
            opcode: 0x41,
            prefix_cb: false,
            name: String::from("LD B,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x42 => Instruction {
            opcode: 0x42,
            prefix_cb: false,
            name: String::from("LD B,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x43 => Instruction {
            opcode: 0x43,
            prefix_cb: false,
            name: String::from("LD B,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x44 => Instruction {
            opcode: 0x44,
            prefix_cb: false,
            name: String::from("LD B,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x45 => Instruction {
            opcode: 0x45,
            prefix_cb: false,
            name: String::from("LD B,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x46 => Instruction {
            opcode: 0x46,
            prefix_cb: false,
            name: String::from("LD B,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x47 => Instruction {
            opcode: 0x47,
            prefix_cb: false,
            name: String::from("LD B,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x48 => Instruction {
            opcode: 0x48,
            prefix_cb: false,
            name: String::from("LD C,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x49 => Instruction {
            opcode: 0x49,
            prefix_cb: false,
            name: String::from("LD C,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x4a => Instruction {
            opcode: 0x4a,
            prefix_cb: false,
            name: String::from("LD C,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x4b => Instruction {
            opcode: 0x4b,
            prefix_cb: false,
            name: String::from("LD C,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x4c => Instruction {
            opcode: 0x4c,
            prefix_cb: false,
            name: String::from("LD C,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x4d => Instruction {
            opcode: 0x4d,
            prefix_cb: false,
            name: String::from("LD C,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x4e => Instruction {
            opcode: 0x4e,
            prefix_cb: false,
            name: String::from("LD C,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x4f => Instruction {
            opcode: 0x4f,
            prefix_cb: false,
            name: String::from("LD C,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x50 => Instruction {
            opcode: 0x50,
            prefix_cb: false,
            name: String::from("LD D,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x51 => Instruction {
            opcode: 0x51,
            prefix_cb: false,
            name: String::from("LD D,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x52 => Instruction {
            opcode: 0x52,
            prefix_cb: false,
            name: String::from("LD D,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x53 => Instruction {
            opcode: 0x53,
            prefix_cb: false,
            name: String::from("LD D,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x54 => Instruction {
            opcode: 0x54,
            prefix_cb: false,
            name: String::from("LD D,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x55 => Instruction {
            opcode: 0x55,
            prefix_cb: false,
            name: String::from("LD D,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x56 => Instruction {
            opcode: 0x56,
            prefix_cb: false,
            name: String::from("LD D,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x57 => Instruction {
            opcode: 0x57,
            prefix_cb: false,
            name: String::from("LD D,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x58 => Instruction {
            opcode: 0x58,
            prefix_cb: false,
            name: String::from("LD E,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x59 => Instruction {
            opcode: 0x59,
            prefix_cb: false,
            name: String::from("LD E,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x5a => Instruction {
            opcode: 0x5a,
            prefix_cb: false,
            name: String::from("LD E,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x5b => Instruction {
            opcode: 0x5b,
            prefix_cb: false,
            name: String::from("LD E,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x5c => Instruction {
            opcode: 0x5c,
            prefix_cb: false,
            name: String::from("LD E,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x5d => Instruction {
            opcode: 0x5d,
            prefix_cb: false,
            name: String::from("LD E,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x5e => Instruction {
            opcode: 0x5e,
            prefix_cb: false,
            name: String::from("LD E,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x5f => Instruction {
            opcode: 0x5f,
            prefix_cb: false,
            name: String::from("LD E,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x60 => Instruction {
            opcode: 0x60,
            prefix_cb: false,
            name: String::from("LD H,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x61 => Instruction {
            opcode: 0x61,
            prefix_cb: false,
            name: String::from("LD H,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x62 => Instruction {
            opcode: 0x62,
            prefix_cb: false,
            name: String::from("LD H,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x63 => Instruction {
            opcode: 0x63,
            prefix_cb: false,
            name: String::from("LD H,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x64 => Instruction {
            opcode: 0x64,
            prefix_cb: false,
            name: String::from("LD H,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x65 => Instruction {
            opcode: 0x65,
            prefix_cb: false,
            name: String::from("LD H,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x66 => Instruction {
            opcode: 0x66,
            prefix_cb: false,
            name: String::from("LD H,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x67 => Instruction {
            opcode: 0x67,
            prefix_cb: false,
            name: String::from("LD H,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x68 => Instruction {
            opcode: 0x68,
            prefix_cb: false,
            name: String::from("LD L,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x69 => Instruction {
            opcode: 0x69,
            prefix_cb: false,
            name: String::from("LD L,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x6a => Instruction {
            opcode: 0x6a,
            prefix_cb: false,
            name: String::from("LD L,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x6b => Instruction {
            opcode: 0x6b,
            prefix_cb: false,
            name: String::from("LD L,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x6c => Instruction {
            opcode: 0x6c,
            prefix_cb: false,
            name: String::from("LD L,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x6d => Instruction {
            opcode: 0x6d,
            prefix_cb: false,
            name: String::from("LD L,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x6e => Instruction {
            opcode: 0x6e,
            prefix_cb: false,
            name: String::from("LD L,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x6f => Instruction {
            opcode: 0x6f,
            prefix_cb: false,
            name: String::from("LD L,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x70 => Instruction {
            opcode: 0x70,
            prefix_cb: false,
            name: String::from("LD (HL),B"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x71 => Instruction {
            opcode: 0x71,
            prefix_cb: false,
            name: String::from("LD (HL),C"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x72 => Instruction {
            opcode: 0x72,
            prefix_cb: false,
            name: String::from("LD (HL),D"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x73 => Instruction {
            opcode: 0x73,
            prefix_cb: false,
            name: String::from("LD (HL),E"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x74 => Instruction {
            opcode: 0x74,
            prefix_cb: false,
            name: String::from("LD (HL),H"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x75 => Instruction {
            opcode: 0x75,
            prefix_cb: false,
            name: String::from("LD (HL),L"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x76 => Instruction {
            opcode: 0x76,
            prefix_cb: false,
            name: String::from("HALT"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x77 => Instruction {
            opcode: 0x77,
            prefix_cb: false,
            name: String::from("LD (HL),A"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x78 => Instruction {
            opcode: 0x78,
            prefix_cb: false,
            name: String::from("LD A,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x79 => Instruction {
            opcode: 0x79,
            prefix_cb: false,
            name: String::from("LD A,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x7a => Instruction {
            opcode: 0x7a,
            prefix_cb: false,
            name: String::from("LD A,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x7b => Instruction {
            opcode: 0x7b,
            prefix_cb: false,
            name: String::from("LD A,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x7c => Instruction {
            opcode: 0x7c,
            prefix_cb: false,
            name: String::from("LD A,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x7d => Instruction {
            opcode: 0x7d,
            prefix_cb: false,
            name: String::from("LD A,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x7e => Instruction {
            opcode: 0x7e,
            prefix_cb: false,
            name: String::from("LD A,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x7f => Instruction {
            opcode: 0x7f,
            prefix_cb: false,
            name: String::from("LD A,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0x80 => Instruction {
            opcode: 0x80,
            prefix_cb: false,
            name: String::from("ADD A,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x81 => Instruction {
            opcode: 0x81,
            prefix_cb: false,
            name: String::from("ADD A,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x82 => Instruction {
            opcode: 0x82,
            prefix_cb: false,
            name: String::from("ADD A,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x83 => Instruction {
            opcode: 0x83,
            prefix_cb: false,
            name: String::from("ADD A,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x84 => Instruction {
            opcode: 0x84,
            prefix_cb: false,
            name: String::from("ADD A,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x85 => Instruction {
            opcode: 0x85,
            prefix_cb: false,
            name: String::from("ADD A,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x86 => Instruction {
            opcode: 0x86,
            prefix_cb: false,
            name: String::from("ADD A,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x87 => Instruction {
            opcode: 0x87,
            prefix_cb: false,
            name: String::from("ADD A,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x88 => Instruction {
            opcode: 0x88,
            prefix_cb: false,
            name: String::from("ADC A,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x89 => Instruction {
            opcode: 0x89,
            prefix_cb: false,
            name: String::from("ADC A,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x8a => Instruction {
            opcode: 0x8a,
            prefix_cb: false,
            name: String::from("ADC A,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x8b => Instruction {
            opcode: 0x8b,
            prefix_cb: false,
            name: String::from("ADC A,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x8c => Instruction {
            opcode: 0x8c,
            prefix_cb: false,
            name: String::from("ADC A,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x8d => Instruction {
            opcode: 0x8d,
            prefix_cb: false,
            name: String::from("ADC A,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x8e => Instruction {
            opcode: 0x8e,
            prefix_cb: false,
            name: String::from("ADC A,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x8f => Instruction {
            opcode: 0x8f,
            prefix_cb: false,
            name: String::from("ADC A,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x90 => Instruction {
            opcode: 0x90,
            prefix_cb: false,
            name: String::from("SUB B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x91 => Instruction {
            opcode: 0x91,
            prefix_cb: false,
            name: String::from("SUB C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x92 => Instruction {
            opcode: 0x92,
            prefix_cb: false,
            name: String::from("SUB D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x93 => Instruction {
            opcode: 0x93,
            prefix_cb: false,
            name: String::from("SUB E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x94 => Instruction {
            opcode: 0x94,
            prefix_cb: false,
            name: String::from("SUB H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x95 => Instruction {
            opcode: 0x95,
            prefix_cb: false,
            name: String::from("SUB L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x96 => Instruction {
            opcode: 0x96,
            prefix_cb: false,
            name: String::from("SUB (HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x97 => Instruction {
            opcode: 0x97,
            prefix_cb: false,
            name: String::from("SUB A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x98 => Instruction {
            opcode: 0x98,
            prefix_cb: false,
            name: String::from("SBC A,B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x99 => Instruction {
            opcode: 0x99,
            prefix_cb: false,
            name: String::from("SBC A,C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x9a => Instruction {
            opcode: 0x9a,
            prefix_cb: false,
            name: String::from("SBC A,D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x9b => Instruction {
            opcode: 0x9b,
            prefix_cb: false,
            name: String::from("SBC A,E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x9c => Instruction {
            opcode: 0x9c,
            prefix_cb: false,
            name: String::from("SBC A,H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x9d => Instruction {
            opcode: 0x9d,
            prefix_cb: false,
            name: String::from("SBC A,L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x9e => Instruction {
            opcode: 0x9e,
            prefix_cb: false,
            name: String::from("SBC A,(HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0x9f => Instruction {
            opcode: 0x9f,
            prefix_cb: false,
            name: String::from("SBC A,A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa0 => Instruction {
            opcode: 0xa0,
            prefix_cb: false,
            name: String::from("AND B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa1 => Instruction {
            opcode: 0xa1,
            prefix_cb: false,
            name: String::from("AND C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa2 => Instruction {
            opcode: 0xa2,
            prefix_cb: false,
            name: String::from("AND D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa3 => Instruction {
            opcode: 0xa3,
            prefix_cb: false,
            name: String::from("AND E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa4 => Instruction {
            opcode: 0xa4,
            prefix_cb: false,
            name: String::from("AND H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa5 => Instruction {
            opcode: 0xa5,
            prefix_cb: false,
            name: String::from("AND L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa6 => Instruction {
            opcode: 0xa6,
            prefix_cb: false,
            name: String::from("AND (HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa7 => Instruction {
            opcode: 0xa7,
            prefix_cb: false,
            name: String::from("AND A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa8 => Instruction {
            opcode: 0xa8,
            prefix_cb: false,
            name: String::from("XOR B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xa9 => Instruction {
            opcode: 0xa9,
            prefix_cb: false,
            name: String::from("XOR C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xaa => Instruction {
            opcode: 0xaa,
            prefix_cb: false,
            name: String::from("XOR D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xab => Instruction {
            opcode: 0xab,
            prefix_cb: false,
            name: String::from("XOR E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xac => Instruction {
            opcode: 0xac,
            prefix_cb: false,
            name: String::from("XOR H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xad => Instruction {
            opcode: 0xad,
            prefix_cb: false,
            name: String::from("XOR L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xae => Instruction {
            opcode: 0xae,
            prefix_cb: false,
            name: String::from("XOR (HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xaf => Instruction {
            opcode: 0xaf,
            prefix_cb: false,
            name: String::from("XOR A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb0 => Instruction {
            opcode: 0xb0,
            prefix_cb: false,
            name: String::from("OR B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb1 => Instruction {
            opcode: 0xb1,
            prefix_cb: false,
            name: String::from("OR C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb2 => Instruction {
            opcode: 0xb2,
            prefix_cb: false,
            name: String::from("OR D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb3 => Instruction {
            opcode: 0xb3,
            prefix_cb: false,
            name: String::from("OR E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb4 => Instruction {
            opcode: 0xb4,
            prefix_cb: false,
            name: String::from("OR H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb5 => Instruction {
            opcode: 0xb5,
            prefix_cb: false,
            name: String::from("OR L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb6 => Instruction {
            opcode: 0xb6,
            prefix_cb: false,
            name: String::from("OR (HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb7 => Instruction {
            opcode: 0xb7,
            prefix_cb: false,
            name: String::from("OR A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb8 => Instruction {
            opcode: 0xb8,
            prefix_cb: false,
            name: String::from("CP B"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xb9 => Instruction {
            opcode: 0xb9,
            prefix_cb: false,
            name: String::from("CP C"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xba => Instruction {
            opcode: 0xba,
            prefix_cb: false,
            name: String::from("CP D"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xbb => Instruction {
            opcode: 0xbb,
            prefix_cb: false,
            name: String::from("CP E"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xbc => Instruction {
            opcode: 0xbc,
            prefix_cb: false,
            name: String::from("CP H"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xbd => Instruction {
            opcode: 0xbd,
            prefix_cb: false,
            name: String::from("CP L"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xbe => Instruction {
            opcode: 0xbe,
            prefix_cb: false,
            name: String::from("CP (HL)"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xbf => Instruction {
            opcode: 0xbf,
            prefix_cb: false,
            name: String::from("CP A"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xc0 => Instruction {
            opcode: 0xc0,
            prefix_cb: false,
            name: String::from("RET NZ"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xc1 => Instruction {
            opcode: 0xc1,
            prefix_cb: false,
            name: String::from("POP BC"),
            bytes: 1,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xc2 => Instruction {
            opcode: 0xc2,
            prefix_cb: false,
            name: String::from("JP NZ,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 4,
            modifies_flags: false
        },
        0xc3 => Instruction {
            opcode: 0xc3,
            prefix_cb: false,
            name: String::from("JP a16"),
            bytes: 3,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xc4 => Instruction {
            opcode: 0xc4,
            prefix_cb: false,
            name: String::from("CALL NZ,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xc5 => Instruction {
            opcode: 0xc5,
            prefix_cb: false,
            name: String::from("PUSH BC"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xc6 => Instruction {
            opcode: 0xc6,
            prefix_cb: false,
            name: String::from("ADD A,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xc7 => Instruction {
            opcode: 0xc7,
            prefix_cb: false,
            name: String::from("RST 00H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xc8 => Instruction {
            opcode: 0xc8,
            prefix_cb: false,
            name: String::from("RET Z"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xc9 => Instruction {
            opcode: 0xc9,
            prefix_cb: false,
            name: String::from("RET"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xca => Instruction {
            opcode: 0xca,
            prefix_cb: false,
            name: String::from("JP Z,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 4,
            modifies_flags: false
        },
        0xcb => Instruction {
            opcode: 0xcb,
            prefix_cb: false,
            name: String::from("PREFIX CB"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcc => Instruction {
            opcode: 0xcc,
            prefix_cb: false,
            name: String::from("CALL Z,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xcd => Instruction {
            opcode: 0xcd,
            prefix_cb: false,
            name: String::from("CALL a16"),
            bytes: 3,
            clocks: 24,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xce => Instruction {
            opcode: 0xce,
            prefix_cb: false,
            name: String::from("ADC A,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcf => Instruction {
            opcode: 0xcf,
            prefix_cb: false,
            name: String::from("RST 08H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xd0 => Instruction {
            opcode: 0xd0,
            prefix_cb: false,
            name: String::from("RET NC"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xd1 => Instruction {
            opcode: 0xd1,
            prefix_cb: false,
            name: String::from("POP DE"),
            bytes: 1,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xd2 => Instruction {
            opcode: 0xd2,
            prefix_cb: false,
            name: String::from("JP NC,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 4,
            modifies_flags: false
        },
        0xd3 => Instruction {
            opcode: 0xd3,
            prefix_cb: false,
            name: String::from("UNKNOWN_D3"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xd4 => Instruction {
            opcode: 0xd4,
            prefix_cb: false,
            name: String::from("CALL NC,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xd5 => Instruction {
            opcode: 0xd5,
            prefix_cb: false,
            name: String::from("PUSH DE"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xd6 => Instruction {
            opcode: 0xd6,
            prefix_cb: false,
            name: String::from("SUB d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xd7 => Instruction {
            opcode: 0xd7,
            prefix_cb: false,
            name: String::from("RST 10H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xd8 => Instruction {
            opcode: 0xd8,
            prefix_cb: false,
            name: String::from("RET C"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xd9 => Instruction {
            opcode: 0xd9,
            prefix_cb: false,
            name: String::from("RETI"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xda => Instruction {
            opcode: 0xda,
            prefix_cb: false,
            name: String::from("JP C,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 4,
            modifies_flags: false
        },
        0xdb => Instruction {
            opcode: 0xdb,
            prefix_cb: false,
            name: String::from("UNKNOWN_DB"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xdc => Instruction {
            opcode: 0xdc,
            prefix_cb: false,
            name: String::from("CALL C,a16"),
            bytes: 3,
            clocks: 12,
            clocks_extra: 12,
            modifies_flags: false
        },
        0xdd => Instruction {
            opcode: 0xdd,
            prefix_cb: false,
            name: String::from("UNKNOWN_DD"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xde => Instruction {
            opcode: 0xde,
            prefix_cb: false,
            name: String::from("SBC A,d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xdf => Instruction {
            opcode: 0xdf,
            prefix_cb: false,
            name: String::from("RST 18H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe0 => Instruction {
            opcode: 0xe0,
            prefix_cb: false,
            name: String::from("LDH (a8),A"),
            bytes: 2,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe1 => Instruction {
            opcode: 0xe1,
            prefix_cb: false,
            name: String::from("POP HL"),
            bytes: 1,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe2 => Instruction {
            opcode: 0xe2,
            prefix_cb: false,
            name: String::from("LD (C),A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe3 => Instruction {
            opcode: 0xe3,
            prefix_cb: false,
            name: String::from("UNKNOWN_E3"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe4 => Instruction {
            opcode: 0xe4,
            prefix_cb: false,
            name: String::from("UNKNOWN_E4"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe5 => Instruction {
            opcode: 0xe5,
            prefix_cb: false,
            name: String::from("PUSH HL"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe6 => Instruction {
            opcode: 0xe6,
            prefix_cb: false,
            name: String::from("AND d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xe7 => Instruction {
            opcode: 0xe7,
            prefix_cb: false,
            name: String::from("RST 20H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xe8 => Instruction {
            opcode: 0xe8,
            prefix_cb: false,
            name: String::from("ADD SP,r8"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xe9 => Instruction {
            opcode: 0xe9,
            prefix_cb: false,
            name: String::from("JP (HL)"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xea => Instruction {
            opcode: 0xea,
            prefix_cb: false,
            name: String::from("LD (a16),A"),
            bytes: 3,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xeb => Instruction {
            opcode: 0xeb,
            prefix_cb: false,
            name: String::from("UNKNOWN_EB"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xec => Instruction {
            opcode: 0xec,
            prefix_cb: false,
            name: String::from("UNKNOWN_EC"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xed => Instruction {
            opcode: 0xed,
            prefix_cb: false,
            name: String::from("UNKNOWN_ED"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xee => Instruction {
            opcode: 0xee,
            prefix_cb: false,
            name: String::from("XOR d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xef => Instruction {
            opcode: 0xef,
            prefix_cb: false,
            name: String::from("RST 28H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf0 => Instruction {
            opcode: 0xf0,
            prefix_cb: false,
            name: String::from("LDH A,(a8)"),
            bytes: 2,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf1 => Instruction {
            opcode: 0xf1,
            prefix_cb: false,
            name: String::from("POP AF"),
            bytes: 1,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xf2 => Instruction {
            opcode: 0xf2,
            prefix_cb: false,
            name: String::from("LD A,(C)"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf3 => Instruction {
            opcode: 0xf3,
            prefix_cb: false,
            name: String::from("DI"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf4 => Instruction {
            opcode: 0xf4,
            prefix_cb: false,
            name: String::from("UNKNOWN_F4"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf5 => Instruction {
            opcode: 0xf5,
            prefix_cb: false,
            name: String::from("PUSH AF"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf6 => Instruction {
            opcode: 0xf6,
            prefix_cb: false,
            name: String::from("OR d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xf7 => Instruction {
            opcode: 0xf7,
            prefix_cb: false,
            name: String::from("RST 30H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xf8 => Instruction {
            opcode: 0xf8,
            prefix_cb: false,
            name: String::from("LD HL,SP+r8"),
            bytes: 2,
            clocks: 12,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xf9 => Instruction {
            opcode: 0xf9,
            prefix_cb: false,
            name: String::from("LD SP,HL"),
            bytes: 1,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xfa => Instruction {
            opcode: 0xfa,
            prefix_cb: false,
            name: String::from("LD A,(a16)"),
            bytes: 3,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xfb => Instruction {
            opcode: 0xfb,
            prefix_cb: false,
            name: String::from("EI"),
            bytes: 1,
            clocks: 4,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xfc => Instruction {
            opcode: 0xfc,
            prefix_cb: false,
            name: String::from("UNKNOWN_FC"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xfd => Instruction {
            opcode: 0xfd,
            prefix_cb: false,
            name: String::from("UNKNOWN_FD"),
            bytes: 0,
            clocks: 0,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xfe => Instruction {
            opcode: 0xfe,
            prefix_cb: false,
            name: String::from("CP d8"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xff => Instruction {
            opcode: 0xff,
            prefix_cb: false,
            name: String::from("RST 38H"),
            bytes: 1,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb00 => Instruction {
            opcode: 0x00,
            prefix_cb: true,
            name: String::from("RLC B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb01 => Instruction {
            opcode: 0x01,
            prefix_cb: true,
            name: String::from("RLC C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb02 => Instruction {
            opcode: 0x02,
            prefix_cb: true,
            name: String::from("RLC D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb03 => Instruction {
            opcode: 0x03,
            prefix_cb: true,
            name: String::from("RLC E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb04 => Instruction {
            opcode: 0x04,
            prefix_cb: true,
            name: String::from("RLC H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb05 => Instruction {
            opcode: 0x05,
            prefix_cb: true,
            name: String::from("RLC L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb06 => Instruction {
            opcode: 0x06,
            prefix_cb: true,
            name: String::from("RLC (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb07 => Instruction {
            opcode: 0x07,
            prefix_cb: true,
            name: String::from("RLC A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb08 => Instruction {
            opcode: 0x08,
            prefix_cb: true,
            name: String::from("RRC B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb09 => Instruction {
            opcode: 0x09,
            prefix_cb: true,
            name: String::from("RRC C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb0a => Instruction {
            opcode: 0x0a,
            prefix_cb: true,
            name: String::from("RRC D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb0b => Instruction {
            opcode: 0x0b,
            prefix_cb: true,
            name: String::from("RRC E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb0c => Instruction {
            opcode: 0x0c,
            prefix_cb: true,
            name: String::from("RRC H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb0d => Instruction {
            opcode: 0x0d,
            prefix_cb: true,
            name: String::from("RRC L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb0e => Instruction {
            opcode: 0x0e,
            prefix_cb: true,
            name: String::from("RRC (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb0f => Instruction {
            opcode: 0x0f,
            prefix_cb: true,
            name: String::from("RRC A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb10 => Instruction {
            opcode: 0x10,
            prefix_cb: true,
            name: String::from("RL B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb11 => Instruction {
            opcode: 0x11,
            prefix_cb: true,
            name: String::from("RL C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb12 => Instruction {
            opcode: 0x12,
            prefix_cb: true,
            name: String::from("RL D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb13 => Instruction {
            opcode: 0x13,
            prefix_cb: true,
            name: String::from("RL E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb14 => Instruction {
            opcode: 0x14,
            prefix_cb: true,
            name: String::from("RL H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb15 => Instruction {
            opcode: 0x15,
            prefix_cb: true,
            name: String::from("RL L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb16 => Instruction {
            opcode: 0x16,
            prefix_cb: true,
            name: String::from("RL (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb17 => Instruction {
            opcode: 0x17,
            prefix_cb: true,
            name: String::from("RL A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb18 => Instruction {
            opcode: 0x18,
            prefix_cb: true,
            name: String::from("RR B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb19 => Instruction {
            opcode: 0x19,
            prefix_cb: true,
            name: String::from("RR C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb1a => Instruction {
            opcode: 0x1a,
            prefix_cb: true,
            name: String::from("RR D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb1b => Instruction {
            opcode: 0x1b,
            prefix_cb: true,
            name: String::from("RR E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb1c => Instruction {
            opcode: 0x1c,
            prefix_cb: true,
            name: String::from("RR H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb1d => Instruction {
            opcode: 0x1d,
            prefix_cb: true,
            name: String::from("RR L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb1e => Instruction {
            opcode: 0x1e,
            prefix_cb: true,
            name: String::from("RR (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb1f => Instruction {
            opcode: 0x1f,
            prefix_cb: true,
            name: String::from("RR A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb20 => Instruction {
            opcode: 0x20,
            prefix_cb: true,
            name: String::from("SLA B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb21 => Instruction {
            opcode: 0x21,
            prefix_cb: true,
            name: String::from("SLA C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb22 => Instruction {
            opcode: 0x22,
            prefix_cb: true,
            name: String::from("SLA D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb23 => Instruction {
            opcode: 0x23,
            prefix_cb: true,
            name: String::from("SLA E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb24 => Instruction {
            opcode: 0x24,
            prefix_cb: true,
            name: String::from("SLA H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb25 => Instruction {
            opcode: 0x25,
            prefix_cb: true,
            name: String::from("SLA L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb26 => Instruction {
            opcode: 0x26,
            prefix_cb: true,
            name: String::from("SLA (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb27 => Instruction {
            opcode: 0x27,
            prefix_cb: true,
            name: String::from("SLA A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb28 => Instruction {
            opcode: 0x28,
            prefix_cb: true,
            name: String::from("SRA B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb29 => Instruction {
            opcode: 0x29,
            prefix_cb: true,
            name: String::from("SRA C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb2a => Instruction {
            opcode: 0x2a,
            prefix_cb: true,
            name: String::from("SRA D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb2b => Instruction {
            opcode: 0x2b,
            prefix_cb: true,
            name: String::from("SRA E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb2c => Instruction {
            opcode: 0x2c,
            prefix_cb: true,
            name: String::from("SRA H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb2d => Instruction {
            opcode: 0x2d,
            prefix_cb: true,
            name: String::from("SRA L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb2e => Instruction {
            opcode: 0x2e,
            prefix_cb: true,
            name: String::from("SRA (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb2f => Instruction {
            opcode: 0x2f,
            prefix_cb: true,
            name: String::from("SRA A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb30 => Instruction {
            opcode: 0x30,
            prefix_cb: true,
            name: String::from("SWAP B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb31 => Instruction {
            opcode: 0x31,
            prefix_cb: true,
            name: String::from("SWAP C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb32 => Instruction {
            opcode: 0x32,
            prefix_cb: true,
            name: String::from("SWAP D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb33 => Instruction {
            opcode: 0x33,
            prefix_cb: true,
            name: String::from("SWAP E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb34 => Instruction {
            opcode: 0x34,
            prefix_cb: true,
            name: String::from("SWAP H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb35 => Instruction {
            opcode: 0x35,
            prefix_cb: true,
            name: String::from("SWAP L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb36 => Instruction {
            opcode: 0x36,
            prefix_cb: true,
            name: String::from("SWAP (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb37 => Instruction {
            opcode: 0x37,
            prefix_cb: true,
            name: String::from("SWAP A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb38 => Instruction {
            opcode: 0x38,
            prefix_cb: true,
            name: String::from("SRL B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb39 => Instruction {
            opcode: 0x39,
            prefix_cb: true,
            name: String::from("SRL C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb3a => Instruction {
            opcode: 0x3a,
            prefix_cb: true,
            name: String::from("SRL D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb3b => Instruction {
            opcode: 0x3b,
            prefix_cb: true,
            name: String::from("SRL E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb3c => Instruction {
            opcode: 0x3c,
            prefix_cb: true,
            name: String::from("SRL H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb3d => Instruction {
            opcode: 0x3d,
            prefix_cb: true,
            name: String::from("SRL L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb3e => Instruction {
            opcode: 0x3e,
            prefix_cb: true,
            name: String::from("SRL (HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb3f => Instruction {
            opcode: 0x3f,
            prefix_cb: true,
            name: String::from("SRL A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb40 => Instruction {
            opcode: 0x40,
            prefix_cb: true,
            name: String::from("BIT 0,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb41 => Instruction {
            opcode: 0x41,
            prefix_cb: true,
            name: String::from("BIT 0,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb42 => Instruction {
            opcode: 0x42,
            prefix_cb: true,
            name: String::from("BIT 0,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb43 => Instruction {
            opcode: 0x43,
            prefix_cb: true,
            name: String::from("BIT 0,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb44 => Instruction {
            opcode: 0x44,
            prefix_cb: true,
            name: String::from("BIT 0,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb45 => Instruction {
            opcode: 0x45,
            prefix_cb: true,
            name: String::from("BIT 0,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb46 => Instruction {
            opcode: 0x46,
            prefix_cb: true,
            name: String::from("BIT 0,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb47 => Instruction {
            opcode: 0x47,
            prefix_cb: true,
            name: String::from("BIT 0,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb48 => Instruction {
            opcode: 0x48,
            prefix_cb: true,
            name: String::from("BIT 1,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb49 => Instruction {
            opcode: 0x49,
            prefix_cb: true,
            name: String::from("BIT 1,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb4a => Instruction {
            opcode: 0x4a,
            prefix_cb: true,
            name: String::from("BIT 1,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb4b => Instruction {
            opcode: 0x4b,
            prefix_cb: true,
            name: String::from("BIT 1,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb4c => Instruction {
            opcode: 0x4c,
            prefix_cb: true,
            name: String::from("BIT 1,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb4d => Instruction {
            opcode: 0x4d,
            prefix_cb: true,
            name: String::from("BIT 1,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb4e => Instruction {
            opcode: 0x4e,
            prefix_cb: true,
            name: String::from("BIT 1,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb4f => Instruction {
            opcode: 0x4f,
            prefix_cb: true,
            name: String::from("BIT 1,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb50 => Instruction {
            opcode: 0x50,
            prefix_cb: true,
            name: String::from("BIT 2,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb51 => Instruction {
            opcode: 0x51,
            prefix_cb: true,
            name: String::from("BIT 2,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb52 => Instruction {
            opcode: 0x52,
            prefix_cb: true,
            name: String::from("BIT 2,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb53 => Instruction {
            opcode: 0x53,
            prefix_cb: true,
            name: String::from("BIT 2,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb54 => Instruction {
            opcode: 0x54,
            prefix_cb: true,
            name: String::from("BIT 2,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb55 => Instruction {
            opcode: 0x55,
            prefix_cb: true,
            name: String::from("BIT 2,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb56 => Instruction {
            opcode: 0x56,
            prefix_cb: true,
            name: String::from("BIT 2,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb57 => Instruction {
            opcode: 0x57,
            prefix_cb: true,
            name: String::from("BIT 2,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb58 => Instruction {
            opcode: 0x58,
            prefix_cb: true,
            name: String::from("BIT 3,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb59 => Instruction {
            opcode: 0x59,
            prefix_cb: true,
            name: String::from("BIT 3,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb5a => Instruction {
            opcode: 0x5a,
            prefix_cb: true,
            name: String::from("BIT 3,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb5b => Instruction {
            opcode: 0x5b,
            prefix_cb: true,
            name: String::from("BIT 3,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb5c => Instruction {
            opcode: 0x5c,
            prefix_cb: true,
            name: String::from("BIT 3,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb5d => Instruction {
            opcode: 0x5d,
            prefix_cb: true,
            name: String::from("BIT 3,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb5e => Instruction {
            opcode: 0x5e,
            prefix_cb: true,
            name: String::from("BIT 3,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb5f => Instruction {
            opcode: 0x5f,
            prefix_cb: true,
            name: String::from("BIT 3,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb60 => Instruction {
            opcode: 0x60,
            prefix_cb: true,
            name: String::from("BIT 4,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb61 => Instruction {
            opcode: 0x61,
            prefix_cb: true,
            name: String::from("BIT 4,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb62 => Instruction {
            opcode: 0x62,
            prefix_cb: true,
            name: String::from("BIT 4,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb63 => Instruction {
            opcode: 0x63,
            prefix_cb: true,
            name: String::from("BIT 4,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb64 => Instruction {
            opcode: 0x64,
            prefix_cb: true,
            name: String::from("BIT 4,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb65 => Instruction {
            opcode: 0x65,
            prefix_cb: true,
            name: String::from("BIT 4,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb66 => Instruction {
            opcode: 0x66,
            prefix_cb: true,
            name: String::from("BIT 4,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb67 => Instruction {
            opcode: 0x67,
            prefix_cb: true,
            name: String::from("BIT 4,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb68 => Instruction {
            opcode: 0x68,
            prefix_cb: true,
            name: String::from("BIT 5,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb69 => Instruction {
            opcode: 0x69,
            prefix_cb: true,
            name: String::from("BIT 5,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb6a => Instruction {
            opcode: 0x6a,
            prefix_cb: true,
            name: String::from("BIT 5,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb6b => Instruction {
            opcode: 0x6b,
            prefix_cb: true,
            name: String::from("BIT 5,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb6c => Instruction {
            opcode: 0x6c,
            prefix_cb: true,
            name: String::from("BIT 5,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb6d => Instruction {
            opcode: 0x6d,
            prefix_cb: true,
            name: String::from("BIT 5,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb6e => Instruction {
            opcode: 0x6e,
            prefix_cb: true,
            name: String::from("BIT 5,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb6f => Instruction {
            opcode: 0x6f,
            prefix_cb: true,
            name: String::from("BIT 5,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb70 => Instruction {
            opcode: 0x70,
            prefix_cb: true,
            name: String::from("BIT 6,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb71 => Instruction {
            opcode: 0x71,
            prefix_cb: true,
            name: String::from("BIT 6,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb72 => Instruction {
            opcode: 0x72,
            prefix_cb: true,
            name: String::from("BIT 6,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb73 => Instruction {
            opcode: 0x73,
            prefix_cb: true,
            name: String::from("BIT 6,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb74 => Instruction {
            opcode: 0x74,
            prefix_cb: true,
            name: String::from("BIT 6,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb75 => Instruction {
            opcode: 0x75,
            prefix_cb: true,
            name: String::from("BIT 6,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb76 => Instruction {
            opcode: 0x76,
            prefix_cb: true,
            name: String::from("BIT 6,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb77 => Instruction {
            opcode: 0x77,
            prefix_cb: true,
            name: String::from("BIT 6,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb78 => Instruction {
            opcode: 0x78,
            prefix_cb: true,
            name: String::from("BIT 7,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb79 => Instruction {
            opcode: 0x79,
            prefix_cb: true,
            name: String::from("BIT 7,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb7a => Instruction {
            opcode: 0x7a,
            prefix_cb: true,
            name: String::from("BIT 7,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb7b => Instruction {
            opcode: 0x7b,
            prefix_cb: true,
            name: String::from("BIT 7,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb7c => Instruction {
            opcode: 0x7c,
            prefix_cb: true,
            name: String::from("BIT 7,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb7d => Instruction {
            opcode: 0x7d,
            prefix_cb: true,
            name: String::from("BIT 7,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb7e => Instruction {
            opcode: 0x7e,
            prefix_cb: true,
            name: String::from("BIT 7,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb7f => Instruction {
            opcode: 0x7f,
            prefix_cb: true,
            name: String::from("BIT 7,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: true
        },
        0xcb80 => Instruction {
            opcode: 0x80,
            prefix_cb: true,
            name: String::from("RES 0,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb81 => Instruction {
            opcode: 0x81,
            prefix_cb: true,
            name: String::from("RES 0,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb82 => Instruction {
            opcode: 0x82,
            prefix_cb: true,
            name: String::from("RES 0,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb83 => Instruction {
            opcode: 0x83,
            prefix_cb: true,
            name: String::from("RES 0,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb84 => Instruction {
            opcode: 0x84,
            prefix_cb: true,
            name: String::from("RES 0,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb85 => Instruction {
            opcode: 0x85,
            prefix_cb: true,
            name: String::from("RES 0,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb86 => Instruction {
            opcode: 0x86,
            prefix_cb: true,
            name: String::from("RES 0,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb87 => Instruction {
            opcode: 0x87,
            prefix_cb: true,
            name: String::from("RES 0,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb88 => Instruction {
            opcode: 0x88,
            prefix_cb: true,
            name: String::from("RES 1,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb89 => Instruction {
            opcode: 0x89,
            prefix_cb: true,
            name: String::from("RES 1,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb8a => Instruction {
            opcode: 0x8a,
            prefix_cb: true,
            name: String::from("RES 1,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb8b => Instruction {
            opcode: 0x8b,
            prefix_cb: true,
            name: String::from("RES 1,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb8c => Instruction {
            opcode: 0x8c,
            prefix_cb: true,
            name: String::from("RES 1,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb8d => Instruction {
            opcode: 0x8d,
            prefix_cb: true,
            name: String::from("RES 1,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb8e => Instruction {
            opcode: 0x8e,
            prefix_cb: true,
            name: String::from("RES 1,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb8f => Instruction {
            opcode: 0x8f,
            prefix_cb: true,
            name: String::from("RES 1,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb90 => Instruction {
            opcode: 0x90,
            prefix_cb: true,
            name: String::from("RES 2,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb91 => Instruction {
            opcode: 0x91,
            prefix_cb: true,
            name: String::from("RES 2,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb92 => Instruction {
            opcode: 0x92,
            prefix_cb: true,
            name: String::from("RES 2,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb93 => Instruction {
            opcode: 0x93,
            prefix_cb: true,
            name: String::from("RES 2,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb94 => Instruction {
            opcode: 0x94,
            prefix_cb: true,
            name: String::from("RES 2,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb95 => Instruction {
            opcode: 0x95,
            prefix_cb: true,
            name: String::from("RES 2,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb96 => Instruction {
            opcode: 0x96,
            prefix_cb: true,
            name: String::from("RES 2,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb97 => Instruction {
            opcode: 0x97,
            prefix_cb: true,
            name: String::from("RES 2,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb98 => Instruction {
            opcode: 0x98,
            prefix_cb: true,
            name: String::from("RES 3,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb99 => Instruction {
            opcode: 0x99,
            prefix_cb: true,
            name: String::from("RES 3,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb9a => Instruction {
            opcode: 0x9a,
            prefix_cb: true,
            name: String::from("RES 3,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb9b => Instruction {
            opcode: 0x9b,
            prefix_cb: true,
            name: String::from("RES 3,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb9c => Instruction {
            opcode: 0x9c,
            prefix_cb: true,
            name: String::from("RES 3,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb9d => Instruction {
            opcode: 0x9d,
            prefix_cb: true,
            name: String::from("RES 3,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb9e => Instruction {
            opcode: 0x9e,
            prefix_cb: true,
            name: String::from("RES 3,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcb9f => Instruction {
            opcode: 0x9f,
            prefix_cb: true,
            name: String::from("RES 3,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba0 => Instruction {
            opcode: 0xa0,
            prefix_cb: true,
            name: String::from("RES 4,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba1 => Instruction {
            opcode: 0xa1,
            prefix_cb: true,
            name: String::from("RES 4,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba2 => Instruction {
            opcode: 0xa2,
            prefix_cb: true,
            name: String::from("RES 4,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba3 => Instruction {
            opcode: 0xa3,
            prefix_cb: true,
            name: String::from("RES 4,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba4 => Instruction {
            opcode: 0xa4,
            prefix_cb: true,
            name: String::from("RES 4,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba5 => Instruction {
            opcode: 0xa5,
            prefix_cb: true,
            name: String::from("RES 4,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba6 => Instruction {
            opcode: 0xa6,
            prefix_cb: true,
            name: String::from("RES 4,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba7 => Instruction {
            opcode: 0xa7,
            prefix_cb: true,
            name: String::from("RES 4,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba8 => Instruction {
            opcode: 0xa8,
            prefix_cb: true,
            name: String::from("RES 5,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcba9 => Instruction {
            opcode: 0xa9,
            prefix_cb: true,
            name: String::from("RES 5,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbaa => Instruction {
            opcode: 0xaa,
            prefix_cb: true,
            name: String::from("RES 5,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbab => Instruction {
            opcode: 0xab,
            prefix_cb: true,
            name: String::from("RES 5,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbac => Instruction {
            opcode: 0xac,
            prefix_cb: true,
            name: String::from("RES 5,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbad => Instruction {
            opcode: 0xad,
            prefix_cb: true,
            name: String::from("RES 5,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbae => Instruction {
            opcode: 0xae,
            prefix_cb: true,
            name: String::from("RES 5,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbaf => Instruction {
            opcode: 0xaf,
            prefix_cb: true,
            name: String::from("RES 5,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb0 => Instruction {
            opcode: 0xb0,
            prefix_cb: true,
            name: String::from("RES 6,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb1 => Instruction {
            opcode: 0xb1,
            prefix_cb: true,
            name: String::from("RES 6,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb2 => Instruction {
            opcode: 0xb2,
            prefix_cb: true,
            name: String::from("RES 6,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb3 => Instruction {
            opcode: 0xb3,
            prefix_cb: true,
            name: String::from("RES 6,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb4 => Instruction {
            opcode: 0xb4,
            prefix_cb: true,
            name: String::from("RES 6,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb5 => Instruction {
            opcode: 0xb5,
            prefix_cb: true,
            name: String::from("RES 6,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb6 => Instruction {
            opcode: 0xb6,
            prefix_cb: true,
            name: String::from("RES 6,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb7 => Instruction {
            opcode: 0xb7,
            prefix_cb: true,
            name: String::from("RES 6,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb8 => Instruction {
            opcode: 0xb8,
            prefix_cb: true,
            name: String::from("RES 7,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbb9 => Instruction {
            opcode: 0xb9,
            prefix_cb: true,
            name: String::from("RES 7,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbba => Instruction {
            opcode: 0xba,
            prefix_cb: true,
            name: String::from("RES 7,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbbb => Instruction {
            opcode: 0xbb,
            prefix_cb: true,
            name: String::from("RES 7,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbbc => Instruction {
            opcode: 0xbc,
            prefix_cb: true,
            name: String::from("RES 7,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbbd => Instruction {
            opcode: 0xbd,
            prefix_cb: true,
            name: String::from("RES 7,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbbe => Instruction {
            opcode: 0xbe,
            prefix_cb: true,
            name: String::from("RES 7,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbbf => Instruction {
            opcode: 0xbf,
            prefix_cb: true,
            name: String::from("RES 7,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc0 => Instruction {
            opcode: 0xc0,
            prefix_cb: true,
            name: String::from("SET 0,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc1 => Instruction {
            opcode: 0xc1,
            prefix_cb: true,
            name: String::from("SET 0,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc2 => Instruction {
            opcode: 0xc2,
            prefix_cb: true,
            name: String::from("SET 0,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc3 => Instruction {
            opcode: 0xc3,
            prefix_cb: true,
            name: String::from("SET 0,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc4 => Instruction {
            opcode: 0xc4,
            prefix_cb: true,
            name: String::from("SET 0,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc5 => Instruction {
            opcode: 0xc5,
            prefix_cb: true,
            name: String::from("SET 0,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc6 => Instruction {
            opcode: 0xc6,
            prefix_cb: true,
            name: String::from("SET 0,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc7 => Instruction {
            opcode: 0xc7,
            prefix_cb: true,
            name: String::from("SET 0,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc8 => Instruction {
            opcode: 0xc8,
            prefix_cb: true,
            name: String::from("SET 1,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbc9 => Instruction {
            opcode: 0xc9,
            prefix_cb: true,
            name: String::from("SET 1,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbca => Instruction {
            opcode: 0xca,
            prefix_cb: true,
            name: String::from("SET 1,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbcb => Instruction {
            opcode: 0xcb,
            prefix_cb: true,
            name: String::from("SET 1,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbcc => Instruction {
            opcode: 0xcc,
            prefix_cb: true,
            name: String::from("SET 1,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbcd => Instruction {
            opcode: 0xcd,
            prefix_cb: true,
            name: String::from("SET 1,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbce => Instruction {
            opcode: 0xce,
            prefix_cb: true,
            name: String::from("SET 1,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbcf => Instruction {
            opcode: 0xcf,
            prefix_cb: true,
            name: String::from("SET 1,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd0 => Instruction {
            opcode: 0xd0,
            prefix_cb: true,
            name: String::from("SET 2,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd1 => Instruction {
            opcode: 0xd1,
            prefix_cb: true,
            name: String::from("SET 2,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd2 => Instruction {
            opcode: 0xd2,
            prefix_cb: true,
            name: String::from("SET 2,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd3 => Instruction {
            opcode: 0xd3,
            prefix_cb: true,
            name: String::from("SET 2,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd4 => Instruction {
            opcode: 0xd4,
            prefix_cb: true,
            name: String::from("SET 2,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd5 => Instruction {
            opcode: 0xd5,
            prefix_cb: true,
            name: String::from("SET 2,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd6 => Instruction {
            opcode: 0xd6,
            prefix_cb: true,
            name: String::from("SET 2,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd7 => Instruction {
            opcode: 0xd7,
            prefix_cb: true,
            name: String::from("SET 2,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd8 => Instruction {
            opcode: 0xd8,
            prefix_cb: true,
            name: String::from("SET 3,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbd9 => Instruction {
            opcode: 0xd9,
            prefix_cb: true,
            name: String::from("SET 3,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbda => Instruction {
            opcode: 0xda,
            prefix_cb: true,
            name: String::from("SET 3,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbdb => Instruction {
            opcode: 0xdb,
            prefix_cb: true,
            name: String::from("SET 3,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbdc => Instruction {
            opcode: 0xdc,
            prefix_cb: true,
            name: String::from("SET 3,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbdd => Instruction {
            opcode: 0xdd,
            prefix_cb: true,
            name: String::from("SET 3,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbde => Instruction {
            opcode: 0xde,
            prefix_cb: true,
            name: String::from("SET 3,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbdf => Instruction {
            opcode: 0xdf,
            prefix_cb: true,
            name: String::from("SET 3,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe0 => Instruction {
            opcode: 0xe0,
            prefix_cb: true,
            name: String::from("SET 4,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe1 => Instruction {
            opcode: 0xe1,
            prefix_cb: true,
            name: String::from("SET 4,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe2 => Instruction {
            opcode: 0xe2,
            prefix_cb: true,
            name: String::from("SET 4,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe3 => Instruction {
            opcode: 0xe3,
            prefix_cb: true,
            name: String::from("SET 4,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe4 => Instruction {
            opcode: 0xe4,
            prefix_cb: true,
            name: String::from("SET 4,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe5 => Instruction {
            opcode: 0xe5,
            prefix_cb: true,
            name: String::from("SET 4,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe6 => Instruction {
            opcode: 0xe6,
            prefix_cb: true,
            name: String::from("SET 4,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe7 => Instruction {
            opcode: 0xe7,
            prefix_cb: true,
            name: String::from("SET 4,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe8 => Instruction {
            opcode: 0xe8,
            prefix_cb: true,
            name: String::from("SET 5,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbe9 => Instruction {
            opcode: 0xe9,
            prefix_cb: true,
            name: String::from("SET 5,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbea => Instruction {
            opcode: 0xea,
            prefix_cb: true,
            name: String::from("SET 5,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbeb => Instruction {
            opcode: 0xeb,
            prefix_cb: true,
            name: String::from("SET 5,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbec => Instruction {
            opcode: 0xec,
            prefix_cb: true,
            name: String::from("SET 5,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbed => Instruction {
            opcode: 0xed,
            prefix_cb: true,
            name: String::from("SET 5,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbee => Instruction {
            opcode: 0xee,
            prefix_cb: true,
            name: String::from("SET 5,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbef => Instruction {
            opcode: 0xef,
            prefix_cb: true,
            name: String::from("SET 5,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf0 => Instruction {
            opcode: 0xf0,
            prefix_cb: true,
            name: String::from("SET 6,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf1 => Instruction {
            opcode: 0xf1,
            prefix_cb: true,
            name: String::from("SET 6,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf2 => Instruction {
            opcode: 0xf2,
            prefix_cb: true,
            name: String::from("SET 6,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf3 => Instruction {
            opcode: 0xf3,
            prefix_cb: true,
            name: String::from("SET 6,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf4 => Instruction {
            opcode: 0xf4,
            prefix_cb: true,
            name: String::from("SET 6,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf5 => Instruction {
            opcode: 0xf5,
            prefix_cb: true,
            name: String::from("SET 6,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf6 => Instruction {
            opcode: 0xf6,
            prefix_cb: true,
            name: String::from("SET 6,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf7 => Instruction {
            opcode: 0xf7,
            prefix_cb: true,
            name: String::from("SET 6,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf8 => Instruction {
            opcode: 0xf8,
            prefix_cb: true,
            name: String::from("SET 7,B"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbf9 => Instruction {
            opcode: 0xf9,
            prefix_cb: true,
            name: String::from("SET 7,C"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbfa => Instruction {
            opcode: 0xfa,
            prefix_cb: true,
            name: String::from("SET 7,D"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbfb => Instruction {
            opcode: 0xfb,
            prefix_cb: true,
            name: String::from("SET 7,E"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbfc => Instruction {
            opcode: 0xfc,
            prefix_cb: true,
            name: String::from("SET 7,H"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbfd => Instruction {
            opcode: 0xfd,
            prefix_cb: true,
            name: String::from("SET 7,L"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbfe => Instruction {
            opcode: 0xfe,
            prefix_cb: true,
            name: String::from("SET 7,(HL)"),
            bytes: 2,
            clocks: 16,
            clocks_extra: 0,
            modifies_flags: false
        },
        0xcbff => Instruction {
            opcode: 0xff,
            prefix_cb: true,
            name: String::from("SET 7,A"),
            bytes: 2,
            clocks: 8,
            clocks_extra: 0,
            modifies_flags: false
        },
        _ => {
            println!("Fatal error, unrecognized opcode!!");
            Instruction {
                opcode: 0x00,
                prefix_cb: false,
                name: String::from("UNDEFINED"),
                bytes: 0,
                clocks: 0,
                clocks_extra: 0,
                modifies_flags: false
            }
        }
    }
}


pub fn get_flags(full_opcode: u16) -> FlagStatus {
    match full_opcode {
        0x04 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x05 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x07 => FlagStatus{ z: FlagMod::Set(false), n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0x09 => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x0c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x0d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x0f => FlagStatus{ z: FlagMod::Set(false), n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0x14 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x15 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x17 => FlagStatus{ z: FlagMod::Set(false), n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0x19 => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x1c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x1d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x1f => FlagStatus{ z: FlagMod::Set(false), n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0x24 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x25 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x27 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Ignore, h: FlagMod::Set(false), cy: FlagMod::Eval },
        0x29 => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x2c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x2d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x2f => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Set(true), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0x34 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x35 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x37 => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(true) },
        0x39 => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x3c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x3d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Ignore },
        0x3f => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0x80 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x81 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x82 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x83 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x84 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x85 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x86 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x87 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x88 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x89 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x8a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x8b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x8c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x8d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x8e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x8f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x90 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x91 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x92 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x93 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x94 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x95 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x96 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x97 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x98 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x99 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x9a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x9b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x9c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x9d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x9e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0x9f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xa0 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa1 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa2 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa3 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa4 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa5 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa6 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa7 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xa8 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xa9 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xaa => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xab => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xac => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xad => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xae => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xaf => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb0 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb1 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb2 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb3 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb4 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb5 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb6 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb7 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xb8 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xb9 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xba => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xbb => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xbc => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xbd => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xbe => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xbf => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xc6 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xce => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xd6 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xde => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xe6 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Set(false) },
        0xe8 => FlagStatus{ z: FlagMod::Set(false), n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xee => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xf6 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xf8 => FlagStatus{ z: FlagMod::Set(false), n: FlagMod::Set(false), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xfe => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(true), h: FlagMod::Eval, cy: FlagMod::Eval },
        0xcb00 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb01 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb02 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb03 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb04 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb05 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb06 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb07 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb08 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb09 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb0a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb0b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb0c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb0d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb0e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb0f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb10 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb11 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb12 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb13 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb14 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb15 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb16 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb17 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb18 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb19 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb1a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb1b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb1c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb1d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb1e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb1f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb20 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb21 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb22 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb23 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb24 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb25 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb26 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb27 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb28 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb29 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb2a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb2b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb2c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb2d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb2e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb2f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb30 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb31 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb32 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb33 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb34 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb35 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb36 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb37 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Set(false) },
        0xcb38 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb39 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb3a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb3b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb3c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb3d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb3e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb3f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(false), cy: FlagMod::Eval },
        0xcb40 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb41 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb42 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb43 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb44 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb45 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb46 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb47 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb48 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb49 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb4a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb4b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb4c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb4d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb4e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb4f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb50 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb51 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb52 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb53 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb54 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb55 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb56 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb57 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb58 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb59 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb5a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb5b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb5c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb5d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb5e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb5f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb60 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb61 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb62 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb63 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb64 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb65 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb66 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb67 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb68 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb69 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb6a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb6b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb6c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb6d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb6e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb6f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb70 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb71 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb72 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb73 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb74 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb75 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb76 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb77 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb78 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb79 => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb7a => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb7b => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb7c => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb7d => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb7e => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        0xcb7f => FlagStatus{ z: FlagMod::Eval, n: FlagMod::Set(false), h: FlagMod::Set(true), cy: FlagMod::Ignore },
        _      => FlagStatus{ z: FlagMod::Ignore, n: FlagMod::Ignore, h: FlagMod::Ignore, cy: FlagMod::Ignore }
    }
}
