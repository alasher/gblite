

struct Instruction {
    opcode: u8,         // The byte opcode of this instruction.
    prefix_cb: bool     // Indicates if this opcode is part of the 0xCB extended instruction set.
    name: String,       // The name of this instruction.
    bytes: u8,          // The total number of bytes of this instruction, including all bytes
                        // required for the opcodes.
    clocks: u8,         // Minimum number of clocks required.
    clocks_extra: u8    // For conditional instructions, the number of extra clocks to take if the
                        // longer instruction path is taken. Ex: JP, RET, etc.
}
