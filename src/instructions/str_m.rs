use crate::memory::Memory;

use super::sign_extend;

pub fn str(instr: u16, reg: &mut [u16], mem: &mut Memory) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 9) & 0x7;
    let offset: u16 = sign_extend(instr & 0x3F, 6);
    mem.mem_write(reg[r1 as usize] + offset, r0);
}