mod compiler;
mod tape;
mod vm;

pub use compiler::{CompileError, compile};
pub use tape::TapeMode;
pub use vm::RuntimeError;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Add(i32),
    Move(i32),
    Read,
    Write,
    JumpIfZero(usize),
    JumpIfNonzero(usize),
}
