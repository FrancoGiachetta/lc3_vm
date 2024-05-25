use std::io::{stdout, Write};

use crate::{memory::Memory, registers::Register};

pub fn putsp(reg: &mut [u16], mem: &mut Memory) {
    let mut index = reg[Register::RR0 as usize];
    let mut c = mem.mem_read(index);

    while c != 0x0000 {
        let c1 = ((c & 0xFF) as u8) as char;
        print!("{}", c1);

        let c2 = ((c >> 8) as u8) as char;

        if c2 != '\0' {
            print!("{}", c2)
        }

        index += 1;

        c = mem.mem_read(index);
    }
    stdout().flush().expect("Flushed");
}
