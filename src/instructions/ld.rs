use crate::{flags::update_flags, memory::Memory, registers::Register};

use super::sign_extend;

pub fn ld(instr: u16, reg: &mut [u16], mem: &mut Memory) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let address = reg[Register::RPC as usize] as u32 + pc_offset as u32;

    reg[r0 as usize] = mem.mem_read(address as u16);

    update_flags(reg, r0);
}
