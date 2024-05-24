use std::io::{stdout, Write};

use crate::registers::Register;

pub fn out(reg: &mut [u16]) {
    print!("{}", (reg[Register::RR0 as usize] as u8) as char);
    stdout().flush().expect("Flushed.");
}
