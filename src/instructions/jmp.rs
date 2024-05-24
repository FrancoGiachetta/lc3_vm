use crate::registers::Register;

pub fn jmp(instr: u16, reg: &mut [u16]) {
    let r1: u16 = (instr >> 6) & 0x7;

    reg[Register::RPC as usize] = reg[r1 as usize];
}
