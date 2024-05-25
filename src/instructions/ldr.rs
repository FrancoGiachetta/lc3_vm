use crate::{flags::update_flags, memory::Memory};

use super::sign_extend;

pub fn ldr(instr: u16, reg: &mut [u16], mem: &mut Memory) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let offset: u16 = sign_extend(instr & 0x3F, 6);
    let address = reg[r1 as usize] + offset;

    reg[r0 as usize] = mem.mem_read(address);
    update_flags(reg, r0);
}
