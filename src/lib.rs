mod compiler;

pub use compiler::{CompileError, Dialect, compile};

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
    labels: [Option<usize>; 26],
}

fn label_index(label: u8) -> usize {
    usize::from(label - b'a')
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Add(i32),
    Move(i32),
    Read,
    Write,
    JumpIfZero(usize),
    JumpIfNonzero(usize),
    Clear,
    Jump(u8),
    Call(u8),
    ReturnOrHalt,
    LoadArgument(usize),
    StoreArgument(usize),
    LoadReturn,
}
