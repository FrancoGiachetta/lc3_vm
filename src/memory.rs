use crate::{
    registers::{check_key, MappedRegister},
    utils::{self, get_char},
};

pub const MEMORY_MAX: usize = 1 << 16;

#[derive(Clone, Copy)]
pub struct Memory {
    pub mem: [u16; MEMORY_MAX],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_MAX],
        }
    }
    pub fn mem_write(&mut self, address: u16, val: u16) {
        self.mem[address as usize] = val;
    }

    pub fn mem_read(&mut self, address: u16) -> u16 {
        if address == MappedRegister::MRKBSR as u16 {
            match check_key() {
                Ok(_) => {
                    self.mem[MappedRegister::MRKBSR as usize] = 1 << 15;
                    self.mem[MappedRegister::MRKBDR as usize] = utils::get_char().unwrap() as u16;
                }
                Err(_) => {
                    self.mem[MappedRegister::MRKBSR as usize] = 0;
                }
            }
        }

        self.mem[address as usize]
    }
}
