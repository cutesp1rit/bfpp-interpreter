use std::{error::Error, fmt};

use crate::{Instruction, Program};

#[derive(Debug)]
pub struct CompileError {
    offset: usize,
    message: String,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "compile error [bf_err_syntax] at {}: {}",
            self.offset, self.message
        )
    }
}

impl Error for CompileError {}

pub fn compile(source: &[u8]) -> Result<Program, CompileError> {
    let mut instructions = Vec::new();
    let mut loops = Vec::new();
    let mut offset = 0;

    while let Some(&byte) = source.get(offset) {
        let instruction = match byte {
            b'+' | b'-' => {
                offset += emit_run(
                    &mut instructions,
                    &source[offset..],
                    b'+',
                    b'-',
                    Instruction::Add,
                );
                continue;
            }
            b'>' | b'<' => {
                offset += emit_run(
                    &mut instructions,
                    &source[offset..],
                    b'>',
                    b'<',
                    Instruction::Move,
                );
                continue;
            }
            b',' => Some(Instruction::Read),
            b'.' => Some(Instruction::Write),
            b'[' => {
                loops.push(instructions.len());
                Some(Instruction::JumpIfZero(0))
            }
            b']' => {
                let open = loops.pop().ok_or_else(|| CompileError {
                    offset,
                    message: format!("unmatched closing bracket at byte {offset}"),
                })?;
                instructions[open] = Instruction::JumpIfZero(instructions.len() + 1);
                Some(Instruction::JumpIfNonzero(open + 1))
            }
            _ => None,
        };
        instructions.extend(instruction);
        offset += 1;
    }

    if !loops.is_empty() {
        return Err(CompileError {
            offset: source.len(),
            message: "unmatched opening bracket".into(),
        });
    }

    Ok(Program { instructions })
}

fn emit_run(
    instructions: &mut Vec<Instruction>,
    source: &[u8],
    positive: u8,
    negative: u8,
    instruction: fn(i32) -> Instruction,
) -> usize {
    let mut delta = 0_i64;
    let mut length = 0;
    for &byte in source {
        if byte == positive {
            delta += 1;
        } else if byte == negative {
            delta -= 1;
        } else {
            break;
        }
        length += 1;
    }

    while delta != 0 {
        let chunk = delta.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        instructions.push(instruction(chunk));
        delta -= i64::from(chunk);
    }
    length
}
