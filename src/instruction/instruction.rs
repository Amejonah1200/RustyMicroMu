use crate::instruction::instruction::AddressingMode::{
    Absolute, Constant0, Constant2, Constant4, Constant8, ConstantN1, ConstantP1, Immediate,
    RegisterDirect, RegisterIndexed, RegisterIndirect, RegisterIndirectAutoincrement, Unknown,
};
use crate::machine::cpu::{ExecutionResult, CPU};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone)]
pub struct Instruction {
    address: u16,
    value: u16,
}

impl Instruction {
    pub fn new(address: u16, value: u16) -> Instruction {
        Instruction { address, value }
    }

    pub fn get_address(&self) -> u16 {
        return self.address;
    }

    pub fn get_value(&self) -> u16 {
        return self.value;
    }
}

/**
* Gives back the addressing mode for the source, caution it assumes that "pc += 2" was done before!
*/
pub fn get_addressing_mode_source(cpu: &CPU, register: u16, source_mode: u16) -> AddressingMode {
    match register {
        0 | 1 | 4..=15 => match source_mode {
            0 => RegisterDirect(register),
            1 => RegisterIndexed(register, get_word_at_pc(cpu)),
            2 => RegisterIndirect(register),
            3 => {
                if register == 0 {
                    Immediate(get_word_at_pc(cpu))
                } else {
                    RegisterIndirectAutoincrement(register)
                }
            }

            _ => Unknown,
        },
        2 => match source_mode {
            0 => RegisterDirect(0),
            1 => Absolute(get_word_at_pc(cpu)),
            2 => Constant4,
            3 => Constant8,
            _ => Unknown,
        },
        3 => match source_mode {
            0 => Constant0,
            1 => ConstantP1,
            2 => Constant2,
            3 => ConstantN1,
            _ => Unknown,
        },
        _ => Unknown,
    }
}

/**
* Gives back the addressing mode for the destination, caution it assumes that "pc += 2" was done before!
*/
pub fn get_addressing_mode_destination(
    cpu: &CPU,
    register: u16,
    destination_mode: u16,
) -> AddressingMode {
    if register == 2 && destination_mode == 1 {
        return Absolute(get_word_at_pc_next(cpu));
    }
    match register {
        0..=15 => match destination_mode {
            0 => RegisterDirect(register),
            1 => RegisterIndexed(register, get_word_at_pc_next(cpu)),
            _ => Unknown,
        },
        _ => Unknown,
    }
}

fn get_word_at_pc(cpu: &CPU) -> u16 {
    cpu.get_memory().get_word_be(cpu.get_pc())
}

fn get_word_at_pc_next(cpu: &CPU) -> u16 {
    cpu.get_memory().get_word_be(cpu.get_pc() + 2)
}

pub trait ExecutableInstruction {
    fn get_extensions_amount(&self) -> u16;
    fn execute(&self, cpu: &mut CPU) -> ExecutionResult;
    fn get_instruction_type(&self) -> InstructionType;
    fn get_instruction_raw(&self) -> Instruction;
}

pub fn is_addressing_mode_extension(mode: &AddressingMode) -> bool {
    match mode {
        RegisterIndexed(_, _) | Absolute(_) => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq, Clone, FromPrimitive, ToPrimitive)]
pub enum InstructionType {
    RRC = 0b00_0000,
    SWPB,
    RRA,
    SXT,
    PUSH,
    CALL,
    RETI,
    JNZ = 0b01_0000,
    JZ,
    JNC,
    JC,
    JN,
    JGE,
    JL,
    JMP,
    MOV = 0b10_0000,
    ADD,
    ADDC,
    SUBC,
    SUB,
    CMP,
    DADD,
    BIT,
    BIC,
    BIS,
    XOR,
    AND,
    Unknown = 0b11_0000,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AddressingMode {
    RegisterDirect(u16),
    RegisterIndexed(u16, u16),
    RegisterIndirect(u16),
    RegisterIndirectAutoincrement(u16),
    Immediate(u16),
    Absolute(u16),
    Constant4,
    Constant8,
    Constant0,
    ConstantP1,
    Constant2,
    ConstantN1,
    Unknown,
}
