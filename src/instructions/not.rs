use crate::flags::update_flags;

pub fn not(instr: u16, reg: &mut [u16]) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    reg[r0 as usize] = !reg[r1 as usize];

    update_flags(reg, r0);
}
