use std::io::{stdout, Write};

use crate::{flags::update_flags, registers::Register, utils};

pub fn in_f(reg: &mut [u16]) {
    print!("Enter a character: ");
    stdout().flush().expect("Flushed");

    let c = utils::get_char().unwrap() as u16;

    reg[Register::RR0 as usize] = c;
}
