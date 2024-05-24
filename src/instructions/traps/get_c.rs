use crate::{flags::update_flags, registers::Register, utils};

pub fn get_c(reg: &mut [u16]) {
    reg[Register::RR0 as usize] = utils::get_char() as u16;

    update_flags(reg, Register::RR0 as u16);
}
