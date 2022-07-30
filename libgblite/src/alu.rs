use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum AluOp {
    Add(bool),
    Sub(bool),
    And,
    Xor,
    Or,
    Comp,
    RotateLeft(bool),
    RotateRight(bool),
    ShiftLeft,
    ShiftRight(bool),
    Swap,
    Test(u8),
    Set(u8, bool)
}

impl fmt::Display for AluOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_name = match *self {
            AluOp::Add(c) => if c { format!("AddCarry") } else { format!("Add") },
            AluOp::Sub(c) => if c { format!("SubCarry") } else { format!("Sub") },
            AluOp::And => format!("And"),
            AluOp::Xor => format!("Xor"),
            AluOp::Or => format!("Or"),
            AluOp::Comp => format!("Comp"),
            AluOp::RotateLeft(c) => if c { format!("RotateLeftCarry") } else { format!("RotateLeft") },
            AluOp::RotateRight(c) => if c { format!("RotateRightCarry") } else { format!("RotateRight") },
            AluOp::ShiftLeft => format!("ShiftLeft"),
            AluOp::ShiftRight(c) => if c { format!("ShiftRightArithmetic") } else { format!("ShiftRightLogical") },
            AluOp::Swap => format!("Swap"),
            AluOp::Test(x) => format!("TestBit{}", x),
            AluOp::Set(x, val) => format!("SetBit{}To{}", x, if val { '1' } else { '0' })
        };

        write!(f, "{}", op_name)
    }
}

pub struct AluInput {
    pub op: AluOp,
    pub op_a: u8,
    pub op_b: u8,
    pub flag_z: bool,
    pub flag_n: bool,
    pub flag_h: bool,
    pub flag_cy: bool,
}

pub struct AluOutput {
    pub result: u8,
    pub flag_z: bool,
    pub flag_n: bool,
    pub flag_h: bool,
    pub flag_cy: bool,
}

pub struct AluInput16 {
    pub subtract: bool,
    pub op_a: u16,
    pub op_b: u16,
    pub flag_z: bool,
    pub flag_n: bool,
    pub flag_h: bool,
    pub flag_cy: bool,
}

pub struct AluOutput16 {
    pub result: u16,
    pub flag_z: bool,
    pub flag_n: bool,
    pub flag_h: bool,
    pub flag_cy: bool,
}

pub fn alu(input: AluInput) -> AluOutput {
    let op_a = input.op_a;
    let op_b = input.op_b;

    let mut z = input.flag_z;
    let mut h = input.flag_h;
    let mut cy = input.flag_cy;

    let result = match input.op {
        AluOp::Add(carry_op) => {
            let cv = if carry_op && input.flag_cy { 1 } else { 0 };
            let (val, over) = op_a.overflowing_add(op_b);
            let (val, overc) = val.overflowing_add(cv);
            cy = over || overc;
            h = (op_a & 0xf).wrapping_add(op_b & 0xf).wrapping_add(cv) > 0xf;
            val
        },
        AluOp::Sub(carry_op) => {
            let cv = if carry_op && input.flag_cy { 1 } else { 0 };
            let (val, over) = op_a.overflowing_sub(op_b);
            let (val, overc) = val.overflowing_sub(cv);
            cy = over || overc;
            h = (op_a & 0xf).wrapping_sub(op_b & 0xf).wrapping_sub(cv) > 0xf;
            val
        },
        AluOp::And      => op_a & op_b,
        AluOp::Xor      => op_a ^ op_b,
        AluOp::Or       => op_a | op_b,
        AluOp::Comp     => {
            let (val, over) = op_a.overflowing_sub(op_b);
            cy = over;
            h = (op_a & 0xf).wrapping_sub(op_b & 0xf).wrapping_sub(0) > 0xf;
            z = val == 0;
            op_a
        },
        AluOp::RotateLeft(carry_op) => {
            let edge_bit = (op_a & 0x80) != 0;
            let rotate_bit = if carry_op { edge_bit } else { input.flag_cy };
            cy = edge_bit;
            if rotate_bit {
                (op_a << 1) | 0x1
            } else {
                op_a << 1
            }
        },
        AluOp::RotateRight(carry_op) => {
            let edge_bit = (op_a & 0x1) != 0;
            let rotate_bit = if carry_op { edge_bit } else { input.flag_cy };
            cy = edge_bit;
            if rotate_bit {
                (op_a >> 1) | 0x80
            } else {
                op_a >> 1
            }
        },
        AluOp::ShiftRight(is_arith) => {
            cy = (op_a & 0x1) != 0;
            let fill_bit = is_arith && (op_a & 0x80 != 0);
            if fill_bit {
                (op_a >> 1) | 0x80
            } else {
                op_a >> 1
            }
        },
        AluOp::ShiftLeft => {
            cy = (op_a & 0x80) != 0;
            op_a << 1
        },
        AluOp::Swap => {
            (op_a & 0xf0).overflowing_shr(4).0 | (op_a & 0xf).overflowing_shl(4).0
        },
        AluOp::Test(off) => {
            z = (op_a & (0x1 << off)) == 0;
            op_a
        },
        AluOp::Set(off, val) => {
            let mask = 0x1 << off;
            if val {
                op_a | mask
            } else {
                op_a & (!mask)
            }
        }
    };

    z = match input.op {
        AluOp::Comp | AluOp::Test(_) => z,
        _ => result == 0
    };

    AluOutput {
        result: result,
        flag_z: z,
        flag_n: input.flag_n,
        flag_h: h,
        flag_cy: cy,
    }
}

// As it turns out, adding/subtracting is really the only 16-bit ALU operation
pub fn alu16(input: AluInput16) -> AluOutput16 {
    let (result, hresult) = if !input.subtract {
        (input.op_a.overflowing_add(input.op_b), (input.op_a & 0xfff).wrapping_add(input.op_b & 0xfff))
    } else {
        (input.op_a.overflowing_sub(input.op_b), (input.op_a & 0xfff).wrapping_sub(input.op_b & 0xfff))
    };

    AluOutput16 {
        result: result.0,
        flag_z: result.0 == 0,
        flag_n: input.flag_n,
        flag_h: hresult > 0xfff,
        flag_cy: result.1,
    }
}