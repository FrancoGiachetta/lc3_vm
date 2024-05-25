use std::io::{stdin, Read};

use crate::{flags::update_flags, registers::Register, utils};

pub fn get_c(reg: &mut [u16]) {
    let mut buffer = [0; 1];
    stdin().read_exact(&mut buffer).unwrap();
    reg[Register::RR0 as usize] = buffer[0] as u16;
}
