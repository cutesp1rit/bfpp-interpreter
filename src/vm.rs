use std::{
    error::Error,
    fmt,
    io::{self, BufRead, Write},
};

use crate::{Instruction, Program, TapeMode, tape::Tape};

#[derive(Debug)]
pub struct RuntimeError {
    pc: usize,
    kind: RuntimeErrorKind,
}

#[derive(Debug)]
enum RuntimeErrorKind {
    TapeBounds,
    Output(io::Error),
    Flush(io::Error),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pc = self.pc;
        match self.kind {
            RuntimeErrorKind::TapeBounds => write!(
                f,
                "runtime error [bf_err_tape_bounds] at pc={pc}: tape move out of bounds at pc={pc}"
            ),
            RuntimeErrorKind::Output(_) => {
                write!(
                    f,
                    "runtime error [bf_err_io] at pc={pc}: output error at pc={pc}"
                )
            }
            RuntimeErrorKind::Flush(_) => {
                write!(f, "runtime error [bf_err_io] at pc={pc}: flush failed")
            }
        }
    }
}

impl Error for RuntimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            RuntimeErrorKind::Output(error) | RuntimeErrorKind::Flush(error) => Some(error),
            RuntimeErrorKind::TapeBounds => None,
        }
    }
}

impl Program {
    pub fn run(
        &self,
        tape_mode: TapeMode,
        input: impl BufRead,
        mut output: impl Write,
    ) -> Result<(), RuntimeError> {
        let mut tape = Tape::new(tape_mode);
        let mut input = input.bytes();
        let mut pc = 0;

        while let Some(&instruction) = self.instructions.get(pc) {
            match instruction {
                Instruction::Add(delta) => tape.set(tape.get().wrapping_add(delta as u8)),
                Instruction::Move(delta) => {
                    if !tape.move_by(delta) {
                        return Err(RuntimeError {
                            pc,
                            kind: RuntimeErrorKind::TapeBounds,
                        });
                    }
                }
                Instruction::Read => tape.set(input.next().and_then(Result::ok).unwrap_or(0)),
                Instruction::Write => {
                    output
                        .write_all(&[tape.get()])
                        .map_err(|error| RuntimeError {
                            pc,
                            kind: RuntimeErrorKind::Output(error),
                        })?;
                }
                Instruction::JumpIfZero(target) if tape.get() == 0 => {
                    pc = target;
                    continue;
                }
                Instruction::JumpIfNonzero(target) if tape.get() != 0 => {
                    pc = target;
                    continue;
                }
                Instruction::JumpIfZero(_) | Instruction::JumpIfNonzero(_) => {}
            }
            pc += 1;
        }

        output.flush().map_err(|error| RuntimeError {
            pc,
            kind: RuntimeErrorKind::Flush(error),
        })
    }
}
