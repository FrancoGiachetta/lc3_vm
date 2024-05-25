use crate::registers::Register;

use super::sign_extend;

pub fn jsr(instr: u16, reg: &mut [u16]) {
    let long_flag: u16 = (instr >> 11) & 1;

    reg[Register::RR7 as usize] = reg[Register::RPC as usize];

    if long_flag != 0 {
        let pc_offset: u16 = sign_extend(instr & 0x7FF, 11);
        reg[Register::RPC as usize] += pc_offset;
    } else {
        let r1 = (instr >> 6) & 0x7;

        reg[Register::RPC as usize] = reg[r1 as usize];
    }
}
