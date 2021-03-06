// Operation codes
pub const OP_R_TYPE: u8 = 0;
pub const OP_BCOND: u8 = 0x01;
pub const OP_J: u8 = 0x02;
pub const OP_JAL: u8 = 0x03;
pub const OP_BEQ: u8 = 0x04;
pub const OP_BNE: u8 = 0x05;
pub const OP_BLEZ: u8 = 0x06;
pub const OP_BGTZ: u8 = 0x07;
pub const OP_ADDI: u8 = 0x08;
pub const OP_ADDIU: u8 = 0x09;
pub const OP_SLTI: u8 = 0x0A;
pub const OP_SLTIU: u8 = 0x0B;
pub const OP_ANDI: u8 = 0x0C;
pub const OP_ORI: u8 = 0x0D;
pub const OP_XORI: u8 = 0x0E;
pub const OP_LUI: u8 = 0x0F;
pub const OP_LB: u8 = 0x20;
pub const OP_LH: u8 = 0x21;
pub const OP_LWL: u8 = 0x22;
pub const OP_LW: u8 = 0x23;
pub const OP_LBU: u8 = 0x24;
pub const OP_LHU: u8 = 0x25;
pub const OP_LWR: u8 = 0x26;
pub const OP_SB: u8 = 0x28;
pub const OP_SH: u8 = 0x29;
pub const OP_SWL: u8 = 0x2A;
pub const OP_SW: u8 = 0x2B;
pub const OP_SWR: u8 = 0x2E;

// R-type function codes
pub const FUNCTION_SLL: u8 = 0x00;
pub const FUNCTION_SRL: u8 = 0x02;
pub const FUNCTION_SRA: u8 = 0x03;
pub const FUNCTION_SLLV: u8 = 0x04;
pub const FUNCTION_SRLV: u8 = 0x06;
pub const FUNCTION_SRAV: u8 = 0x07;
pub const FUNCTION_JR: u8 = 0x08;
pub const FUNCTION_JALR: u8 = 0x09;
pub const FUNCTION_SYSCALL: u8 = 0x0C;
pub const FUNCTION_BREAK: u8 = 0x0D;
pub const FUNCTION_MFHI: u8 = 0x10;
pub const FUNCTION_MTHI: u8 = 0x11;
pub const FUNCTION_MFLO: u8 = 0x12;
pub const FUNCTION_MTLO: u8 = 0x13;
pub const FUNCTION_MULT: u8 = 0x18;
pub const FUNCTION_MULTU: u8 = 0x19;
pub const FUNCTION_DIV: u8 = 0x1A;
pub const FUNCTION_DIVU: u8 = 0x1B;
pub const FUNCTION_ADD: u8 = 0x20;
pub const FUNCTION_ADDU: u8 = 0x21;
pub const FUNCTION_SUB: u8 = 0x22;
pub const FUNCTION_SUBU: u8 = 0x23;
pub const FUNCTION_AND: u8 = 0x24;
pub const FUNCTION_OR: u8 = 0x25;
pub const FUNCTION_XOR: u8 = 0x26;
pub const FUNCTION_NOR: u8 = 0x27;
pub const FUNCTION_SLT: u8 = 0x2A;
pub const FUNCTION_SLTU: u8 = 0x2B;

// OP_BCOND rt values
pub const BCOND_RT_BLTZ: u8 = 0x00;
pub const BCOND_RT_BGEZ: u8 = 0x01;
pub const BCOND_RT_BLTZAL: u8 = 0x10;
pub const BCOND_RT_BGEZAL: u8 = 0x11;

// Register numbers
pub const REG_V0: u8 = 2;
pub const REG_A0: u8 = 4;
pub const REG_A1: u8 = 5;
/// The stack pointer register
pub const REG_SP: u8 = 29;
pub const REG_RA: u8 = 31;

pub static REGISTER_NAMES: [&str; 32] = [
    "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1", "$t2", "$t3", "$t4",
    "$t5", "$t6", "$t7", "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7", "$t8", "$t9",
    "$k0", "$k1", "$gp", "$sp", "$fp", "$ra",
];

// Syscall codes
pub const SYSCALL_PRINT_INT: u32 = 1;
pub const SYSCALL_PRINT_STR: u32 = 4;
pub const SYSCALL_READ_INT: u32 = 5;
pub const SYSCALL_READ_STRING: u32 = 8;
pub const SYSCALL_EXIT: u32 = 10;
pub const SYSCALL_EXIT2: u32 = 17;

// Memory offsets
/// The bottom of the stack
pub const STACK_BOTTOM: u32 = 0x7fffefff;
pub const TEXT_OFFSET: u32 = 0x00400000;
pub const DATA_OFFSET: u32 = 0x10000000;

/// The entrypoint of R2K programs
pub const R2K_ENTRYPOINT: &str = "__r2k__entry__";

// Symbol flags
/// We have seen the definition of this symbol (is not a global import)
pub const SYM_DEF_SEEN: u32 = 0x20;
/// This symbol defines a label
pub const SYM_DEF_LABEL: u32 = 0x80;
/// This symbol was defined as global
pub const SYM_GLOBAL: u32 = 0x4000;
pub const SYM_MODE_MASK: u32 = 0xF;

// Relocation flags
pub const REL_LOWER_IMM: u8 = 0x01;
pub const REL_SPLIT_IMM: u8 = 0x02;
pub const REL_WORD: u8 = 0x03;
pub const REL_JUMP: u8 = 0x04;
pub const REL_UPPER_IMM: u8 = 0x05;

// Reference flags
pub const REF_METHOD_ADD: u8 = 0x00;
pub const REF_METHOD_REPLACE: u8 = 0x10;
pub const REF_METHOD_SUBTRACT: u8 = 0x20;
pub const REF_METHOD_MASK: u8 = 0x30;
pub const REF_TARGET_IMM: u8 = 0x01;
pub const REF_TARGET_HALF_WORD: u8 = 0x02;
pub const REF_TARGET_SPLIT_IMM: u8 = 0x03;
pub const REF_TARGET_WORD: u8 = 0x04;
pub const REF_TARGET_JUMP: u8 = 0x05;
pub const REF_TARGET_MASK: u8 = 0x0F;
