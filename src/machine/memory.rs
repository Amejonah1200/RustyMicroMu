#![allow(dead_code)]

pub struct Memory {
    memory: [u8; 0xFFFF]
}

impl Memory {
    pub fn set_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn get_byte(&mut self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn set_word(&mut self, address: u16, value: u16) {
        self.set_byte(address, value as u8);
        self.set_byte(address + 1, (value >> 8) as u8);
    }

    pub fn get_word(&mut self, address: u16) -> u16 {
        return (self.get_byte(address) | (self.get_byte(address + 1) << 8)) as u16;
    }

    pub fn new() -> Self {
        Memory {
            memory: [0; 0xFFFF],
        }
    }
}
