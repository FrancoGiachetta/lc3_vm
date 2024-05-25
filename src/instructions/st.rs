use crate::{memory::Memory, registers::Register};

use super::sign_extend;

pub fn st(instr: u16, reg: &mut [u16], mem: &mut Memory) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let address = reg[Register::RPC as usize] + pc_offset;

    mem.mem_write(address, reg[r0 as usize]);
}
