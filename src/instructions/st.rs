use crate::{memory::Memory, registers::Register};

use super::sign_extend;

pub fn st(instr: u16, reg: &mut [u16], mem: &mut Memory) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);

    let mem_address: u16 = (reg[Register::RPC as usize] as u32 + pc_offset as u32) as u16;

    mem.mem_write(mem_address, reg[r0 as usize]);
}