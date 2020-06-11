#![allow(dead_code)]

use crate::instruction::disassembler;
use crate::instruction::disassembler::ParseState;
use crate::instruction::instruction::Instruction;
use crate::machine::cpu::ExecutionResult::CpuOff;
use crate::machine::memory::Memory;

pub struct CPU {
    pub memory: Memory,
    registers: [u16; 0x10],
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            memory,
            registers: [0; 0x10],
        }
    }

    fn run(&mut self) -> ExecutionResult {
        loop {
            if self.is_flag_set(StatusFlag::CpuOffFlag as u16) {
                break;
            }
            let instruction =
                Instruction::new(self.get_pc(), self.memory.get_word_be(self.get_pc()));
            let parse_state = disassembler::parse_instruction(self, instruction);
            let executable = match parse_state {
                ParseState::Done(insn) => insn,
                ParseState::Error(_) => return ExecutionResult::ParseError,
            };
            self.set_pc(self.get_pc() + (executable.get_extensions_amount() as u16) * 2);
            executable.execute(self);
        }
        CpuOff
    }

    pub fn is_flag_set(&self, flag: u16) -> bool {
        return (self.get_register(CPURegister::SR as usize) & flag) != 0;
    }

    pub fn set_flag(&mut self, flag: u16, value: bool) {
        self.set_sr(if value {
            self.get_sr() | flag
        } else {
            self.get_sr() & !flag
        });
    }

    pub fn toggle_flag(&mut self, flag: u16) {
        self.set_register(CPURegister::SR as usize, (self.get_sr() ^ flag) & 0xff);
    }

    pub fn get_pc(&self) -> u16 {
        return self.get_register(CPURegister::PC as usize);
    }

    pub fn set_pc(&mut self, value: u16) {
        self.set_register(CPURegister::PC as usize, value);
    }

    pub fn get_sp(&self) -> u16 {
        return self.get_register(CPURegister::SP as usize);
    }
    pub fn set_sp(&mut self, value: u16) {
        self.set_register(CPURegister::SP as usize, value);
    }

    pub fn get_sr(&self) -> u16 {
        return self.get_register(CPURegister::SR as usize);
    }
    pub fn set_sr(&mut self, value: u16) {
        self.set_register(CPURegister::SR as usize, value);
    }

    pub fn get_register(&self, register: usize) -> u16 {
        return self.registers[register];
    }

    pub fn set_register(&mut self, register: usize, value: u16) {
        self.registers[register] = value;
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
}

pub enum ExecutionResult {
    Done,
    CpuOff,
    ParseError,
    ExecutionError,
}

#[repr(u8)]
pub enum CPURegister {
    /**
     * The 16-bit program counter (PC/R0) points to the next instruction to be executed.
     */
    PC,
    /**
    * The stack pointer (SP/R1) is used by the CPU to store the return addresses of subroutine
    calls and interrupts.
    */
    SP,
    /**
    *The status register (SR/R2),used as a source or destination register, can be used in
    the register mode only addressed with word instructions.
    */
    SR,
    CG,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[repr(u16)]
pub enum StatusFlag {
    /**
     * Carry bit. Set when the result of a byte or word operation produced a carry and cleared when no carry occurred.
     */
    CarryFlag = 1 << 0,
    /**
     * Zero bit. Set when the result of a byte or word operation is 0 and cleared when the result is not 0.
     */
    ZeroFlag = 1 << 1,
    /**
     * Negative bit. Set when the result of a byte or word operation is negative and cleared when the result is not negative.<br>
     * * Word operation: N is set to the value of bit 15 of the result.<br>
     * * Byte operation: N is set to the value of bit 7 of the result.<br>
     */
    NegativeFlag = 1 << 2,
    /**
     * General interrupt enable. When set, enables maskable interrupts.When reset, all maskable interrupts are disabled.
     */
    GeneralInterruptEnabledFlag = 1 << 3,
    /**
     * CPU off. When set, turns off the CPU.
     */
    CpuOffFlag = 1 << 4,
    /**
     * Oscillator Off.<br>
     * When set, turns off the LFXT1 crystal oscillator, when LFXT1CLK is not use for MCLK or SMCLK.
     */
    OscillatorOffFlag = 1 << 5,
    /**
     * System clock generator 0. When set, turns off the DCO dc generator, if DCO CLK is not used for MCLK or SMCLK.
     */
    SystemClockGenerator0Of = 1 << 6,
    /**
     * System clock generator 1. When set, turns off the SMCLK.
     */
    SystemClockGenerator1Off = 1 << 7,
    /**
     * Overflow bit. This bit is set when the result of an arithmetic operation overflows the signed-variable range.<br>
     * ADD(.B), ADDC(.B) -> Set when:<br>
     * * Positive + Positive = Negative<br>
     * * Negative + Negative = Positive<br>
     * * Otherwise reset<br>
     * SUB(.B), SUBC(.B), CMP(.B) -> Set when:<br>
     * * Positive – Negative = Negative<br>
     * * Negative – Positive = Positive<br>
     * * Otherwise reset<br>
     */
    OverflowFlag = 1 << 8,
}
