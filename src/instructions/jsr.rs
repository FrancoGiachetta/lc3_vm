use crate::registers::Register;

use super::sign_extend;

pub fn jsr(instr: u16, reg: &mut [u16]) {
    let long_flag: u16 = (instr >> 11) & 1;

    reg[Register::RR7 as usize] = reg[Register::RPC as usize];

    if long_flag == 1 {
        let pc_offset: u16 = sign_extend(instr & 0x7FF, 11);
        let val = reg[Register::RPC as usize] as u32 + pc_offset as u32;

        reg[Register::RPC as usize] = val as u16;
    } else {
        let r1 = (instr >> 6) & 0x7;

        reg[Register::RPC as usize] = reg[r1 as usize];
    }
}
