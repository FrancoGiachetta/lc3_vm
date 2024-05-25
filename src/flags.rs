use crate::registers::Register;

pub enum Flags {
    FLPOS = 1 << 0, /* P */
    FLZRO = 1 << 1, /* Z */
    FLNEG = 1 << 2, /* N */
}

pub fn update_flags(regs: &mut [u16], reg: u16) {
    if regs[reg as usize] == 0 {
        regs[Register::RCOND as usize] = Flags::FLZRO as u16;
    } else if (regs[reg as usize] >> 15) != 0 {
        regs[Register::RCOND as usize] = Flags::FLNEG as u16;
    } else {
        regs[Register::RCOND as usize] = Flags::FLPOS as u16;
    }
}
