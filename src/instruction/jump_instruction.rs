#![allow(dead_code)]

use num::FromPrimitive;

use crate::instruction::disassembler::ParseState;
use crate::instruction::instruction::{ExecutableInstruction, Instruction, InstructionType};
use crate::machine::cpu::ExecutionResult::{Done, ParseError};
use crate::machine::cpu::StatusFlag::{CarryFlag, NegativeFlag, OverflowFlag, ZeroFlag};
use crate::machine::cpu::{ExecutionResult, CPU};

pub struct JumpInstruction {
    instruction: Instruction,
    jump_type: JumpType,
    pub offset: u16,
}

impl JumpInstruction {
    pub fn new(instruction: Instruction) -> ParseState {
        let jump = JumpInstruction {
            jump_type: get_jump_from_instruction(instruction.get_value()),
            offset: {
                let offset = instruction.get_value() & 0x3ff;
                if offset & 0x200 != 0 {
                    offset - 0x400
                } else {
                    offset
                }
            },
            instruction,
        };
        if jump.jump_type == JumpType::Unknown {
            return ParseState::Error(jump.instruction);
        }
        ParseState::Done(Box::new(jump))
    }

    pub fn get_jump_type(&self) -> JumpType {
        self.jump_type.clone()
    }

    pub fn get_instruction(&self) -> Instruction {
        self.instruction.clone()
    }
}

impl ExecutableInstruction for JumpInstruction {
    fn get_extensions_amount(&self) -> u16 {
        0
    }

    fn execute(&self, cpu: &mut CPU) -> ExecutionResult {
        if self.jump_type == JumpType::Unknown {
            return ParseError;
        }
        let can_jump = match self.get_jump_type() {
            JumpType::JNZ => !cpu.is_flag_set(ZeroFlag as u16),
            JumpType::JZ => cpu.is_flag_set(ZeroFlag as u16),
            JumpType::JNC => !cpu.is_flag_set(CarryFlag as u16),
            JumpType::JC => cpu.is_flag_set(CarryFlag as u16),
            JumpType::JN => !cpu.is_flag_set(NegativeFlag as u16),
            JumpType::JGE => {
                cpu.is_flag_set(NegativeFlag as u16) == cpu.is_flag_set(OverflowFlag as u16)
            }
            JumpType::JL => {
                cpu.is_flag_set(NegativeFlag as u16) != cpu.is_flag_set(OverflowFlag as u16)
            }
            JumpType::JMP => true,
            _ => false,
        };
        if can_jump {
            cpu.set_pc(cpu.get_pc() + self.offset * 2);
        }
        Done
    }

    fn get_instruction_type(&self) -> InstructionType {
        let type_id = (1 << 4) | (self.jump_type.clone() as u16);
        if (16u16 <= type_id) && (type_id < 31u16) {
            match InstructionType::from_u16(type_id) {
                Some(jump) => jump,
                None => InstructionType::Unknown,
            }
        } else {
            InstructionType::Unknown
        }
    }

    fn get_instruction_raw(&self) -> Instruction {
        self.instruction.clone()
    }
}

pub fn get_jump_from_instruction(value: u16) -> JumpType {
    match JumpType::from_u16((value >> 10) & 0b111) {
        Some(jump) => jump,
        None => JumpType::Unknown,
    }
}

enum_from_primitive! {
    # [derive(Debug, PartialEq, Clone)]
    pub enum JumpType {
        JNZ,
        JZ,
        JNC,
        JC,
        JN,
        JGE,
        JL,
        JMP,
        Unknown,
    }
}
