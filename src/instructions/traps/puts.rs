use std::io::{stdout, Write};

use crate::{memory::Memory, registers::Register};

pub fn puts(reg: &mut [u16], mem: &mut Memory) {
    let mut index = reg[Register::RR0 as usize];
    let mut c = mem.mem_read(index);

    while c != 0x0000 {
        print!("{}", (c as u8) as char);
        index += 1;

        c = mem.mem_read(index);
    }
    stdout().flush().expect("Flushed");
}
