use crate::instruction::disassembler::ParseState::Error;
use crate::instruction::instruction::{ExecutableInstruction, Instruction};
use crate::machine::cpu::CPU;

pub fn parse_instruction(cpu: &CPU, instruction: Instruction) -> ParseState {
    match instruction.get_value() >> 13 {
        // 0 => Done(2),     // Single Operand
        // 1 => Done(2),     // Jump
        // 2..=7 => Done(2), // Double Operand
        _ => Error(instruction),
    }
}

pub enum ParseState {
    Done(Box<dyn ExecutableInstruction>),
    Error(Instruction),
}
