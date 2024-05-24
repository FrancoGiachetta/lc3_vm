use crate::flags::update_flags;

use super::sign_extend;

pub fn add(instr: u16, reg: &mut [u16]) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);

        let val = reg[r1 as usize] as u32 + imm5 as u32;

        reg[r0 as usize] = val as u16;
    } else {
        let r2: u16 = instr & 0x7;
        let val = reg[r1 as usize] as u32 + reg[r2 as usize] as u32;

        reg[r0 as usize] = val as u16;
    }

    update_flags(reg, r0);
}
