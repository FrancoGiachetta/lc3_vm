mod flags;
mod image;
mod instructions;
mod memory;
mod registers;
mod setup;
mod utils;

use core::panic;
use std::env;

use flags::Flags;
use memory::Memory;
use registers::{Register, R_COUNT};
use setup::restore_input_buffering;

fn main() {
    setup::setup();

    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        /* show usage string */
        panic!("lc3 [image-file1] ...\n");
    }

    for arg in args {
        match image::read_image(arg) {
            Ok(mem) => {
                execute_program(mem);
                restore_input_buffering();
            }
            Err(e) => {
                restore_input_buffering();
                panic!("{e}");
            }
        }
    }
}

fn execute_program(mut memory: Memory) {
    let mut reg: [u16; R_COUNT] = [0; R_COUNT];

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[Register::RCOND as usize] = Flags::FLZRO as u16;

    reg[Register::RPC as usize] = registers::PC_START;

    let mut running = true;

    while running {
        reg[Register::RPC as usize] += 1;
        let instr: u16 = memory.mem_read(reg[Register::RPC as usize]);

        instructions::execute_instruction(&mut memory, &mut reg, instr, &mut running);
    }
}
