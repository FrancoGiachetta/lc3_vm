use std::io::{stdout, Write};

use crate::{flags::update_flags, registers::Register, utils};

pub fn in_f(reg: &mut [u16]) {
    print!("Enter a character : ");
    let c = utils::get_char();
    print!("{}", (c as u8) as char);
    stdout().flush().expect("Flushed.");

    reg[Register::RR0 as usize] = c as u16;
    update_flags(reg, Register::RR0 as u16);
}
