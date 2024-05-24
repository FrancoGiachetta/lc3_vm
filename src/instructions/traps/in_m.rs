use std::io::{stdout, Write};

use crate::{flags::update_flags, registers::Register, utils};

pub fn in_f(reg: &mut [u16]) {
    print!("Enter a character : ");
    stdout().flush().expect("Flushed.");

    reg[Register::RR0 as usize] = utils::get_char() as u16;
    update_flags(reg, Register::RR0 as u16);
}
