use crate::registers::Register;

use super::sign_extend;

pub fn br(instr: u16, reg: &mut [u16]) {
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;

    if cond_flag & reg[Register::RCOND as usize] == 1 {
        reg[Register::RPC as usize] += pc_offset;
    }
}
