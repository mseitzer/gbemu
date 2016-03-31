pub mod instructions;

mod opcodes;
mod registers;

pub use self::instructions::{Instr, Op, Immediate, Condition, Addr};
pub use self::registers::{Reg8, Reg16};

pub use self::instructions::from_opcode;
pub use self::instructions::from_ext_opcode;

pub use self::opcodes::cycles;
pub use self::opcodes::cycles_jmp;