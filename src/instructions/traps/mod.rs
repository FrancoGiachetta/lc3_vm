mod get_c;
mod halt;
mod in_m;
mod out;
mod puts;
mod putsp;

use crate::{memory::Memory, setup::disable_input_buffering};

pub enum Traps {
    TRAPGETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    TRAPOUT = 0x21,   /* output a character */
    TRAPPUTS = 0x22,  /* output a word string */
    TRAPIN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    TRAPPUTSP = 0x24, /* output a byte string */
    TRAPHALT = 0x25,  /* halt the program */
}

pub fn handle_trap(instr: u16, mem: &mut Memory, running: &mut bool, reg: &mut [u16]) {
    disable_input_buffering();

    match instr & 0xFF {
        x if x == Traps::TRAPGETC as u16 => get_c::get_c(reg),
        x if x == Traps::TRAPOUT as u16 => out::out(reg),
        x if x == Traps::TRAPPUTS as u16 => puts::puts(reg, mem),
        x if x == Traps::TRAPIN as u16 => in_m::in_f(reg),
        x if x == Traps::TRAPPUTSP as u16 => putsp::putsp(reg, mem),
        x if x == Traps::TRAPHALT as u16 => halt::halt(running),
        x => panic!("Incorrect trap {}", x),
    }
}
