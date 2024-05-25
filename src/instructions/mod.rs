mod add;
mod and;
mod br;
mod jmp;
mod jsr;
mod ld;
mod ldi;
mod ldr;
mod lea;
mod not;
mod st;
mod sti;
mod str_m;
mod traps;

use crate::{memory::Memory, registers::Register};

#[derive(Clone, Copy)]
pub enum Opcodes {
    OPBR = 0, /* branch */
    OPADD,    /* add  */
    OPLD,     /* load */
    OPST,     /* store */
    OPJSR,    /* jump register */
    OPAND,    /* bitwise and */
    OPLDR,    /* load register */
    OPSTR,    /* store register */
    OPRTI,    /* unused */
    OPNOT,    /* bitwise not */
    OPLDI,    /* load indirect */
    OPSTI,    /* store indirect */
    OPJMP,    /* jump */
    OPRES,    /* reserved (unused) */
    OPLEA,    /* load effective address */
    OPTRAP,   /* execute trap */
}

impl TryFrom<u16> for Opcodes {
    type Error = String;

    fn try_from(value: u16) -> Result<Opcodes, String> {
        match value {
            0 => Ok(Opcodes::OPBR),
            1 => Ok(Opcodes::OPADD),
            2 => Ok(Opcodes::OPLD),
            3 => Ok(Opcodes::OPST),
            4 => Ok(Opcodes::OPJSR),
            5 => Ok(Opcodes::OPAND),
            6 => Ok(Opcodes::OPLDR),
            7 => Ok(Opcodes::OPSTR),
            8 => Ok(Opcodes::OPRTI),
            9 => Ok(Opcodes::OPNOT),
            10 => Ok(Opcodes::OPLDI),
            11 => Ok(Opcodes::OPSTI),
            12 => Ok(Opcodes::OPJMP),
            13 => Ok(Opcodes::OPRES),
            14 => Ok(Opcodes::OPLEA),
            15 => Ok(Opcodes::OPTRAP),
            _ => Err("Incorrect Opcode".to_string()),
        }
    }
}

pub fn execute_instruction(mem: &mut Memory, reg: &mut [u16], instr: u16, running: &mut bool) {
    let op: u16 = instr >> 12;
    /*
        println!("\n OP: {op}");
        println!("REG 0: {}", reg[Register::RR0 as usize]);
        println!("REG 1: {}", reg[Register::RR1 as usize]);
        println!("REG 2: {}", reg[Register::RR2 as usize]);
        println!("REG 3: {}", reg[Register::RR3 as usize]);
        println!("REG 4: {}", reg[Register::RR4 as usize]);
        println!("REG 5: {}", reg[Register::RR5 as usize]);
        println!("REG 6: {}", reg[Register::RR6 as usize]);
        println!("REG 7: {}", reg[Register::RR7 as usize]);
    */
    match op.try_into().unwrap() {
        Opcodes::OPADD => add::add(instr, reg),
        Opcodes::OPAND => and::and(instr, reg),
        Opcodes::OPNOT => not::not(instr, reg),
        Opcodes::OPBR => br::br(instr, reg),
        Opcodes::OPJMP => jmp::jmp(instr, reg),
        Opcodes::OPJSR => jsr::jsr(instr, reg),
        Opcodes::OPLD => ld::ld(instr, reg, mem),
        Opcodes::OPLDI => ldi::ldi(instr, reg, mem),
        Opcodes::OPLDR => ldr::ldr(instr, reg, mem),
        Opcodes::OPLEA => lea::lea(instr, reg),
        Opcodes::OPST => st::st(instr, reg, mem),
        Opcodes::OPSTI => sti::sti(instr, reg, mem),
        Opcodes::OPSTR => str_m::str(instr, reg, mem),
        Opcodes::OPTRAP => traps::handle_trap(instr, mem, running, reg),
        Opcodes::OPRES => {}
        Opcodes::OPRTI => {}
    }
}

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }

    x
}
