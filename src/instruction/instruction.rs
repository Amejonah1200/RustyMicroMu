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

enum_from_primitive! {
# [derive(Debug, PartialEq, Clone)]
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum AddressingModes {
    RegisterDirect(u8),
    RegisterIndexed(u8, u16),
    RegisterIndirect(u8),
    RegisterIndirectAutoincrement(u8),
    Relative(u16),
    Immediate(u16),
    Absolute(u16),
    Constant4,
    Constant8,
    Constant0,
    ConstantP1,
    Constant2,
    ConstantN1,
}
