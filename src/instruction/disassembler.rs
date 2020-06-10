use crate::instruction::instruction::{ExecutableInstruction, Instruction};

pub enum ParseState {
    Done(Box<dyn ExecutableInstruction>),
    Error(Instruction),
}
