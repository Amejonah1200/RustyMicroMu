use crate::machine::memory::Memory;

pub struct CPU {
    pub memory: Memory,
    registers: [u16; 0x10]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new(),
            registers: [0; 0x10]
        }
    }

    pub fn get_register(&self, register: u8) -> u16 {
        return self.registers[register as usize];
    }

    pub fn set_register(&mut self, register: u8, value: u16) {
        self.registers[register as usize] = value;
    }
}
