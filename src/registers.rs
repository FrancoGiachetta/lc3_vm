use std::os::fd::BorrowedFd;

use nix::sys::{select, time::TimeVal};

use libc::STDIN_FILENO;

pub const PC_START: u16 = 0x3000;
pub const R_COUNT: usize = Register::RCOUNT as usize;

pub enum Register {
    RR0,
    RR1,
    RR2,
    RR3,
    RR4,
    RR5,
    RR6,
    RR7,
    RPC,   /* program counter */
    RCOND, /* condition flags */
    RCOUNT,
}

pub enum MappedRegister {
    MRKBSR = 0xFE00, /* keyboard status */
    MRKBDR = 0xFE02, /* keyboard data */
}

pub fn check_key() -> Result<i32, nix::errno::Errno> {
    let mut readfds = select::FdSet::new();
    let stdin_fileno = unsafe { BorrowedFd::borrow_raw(STDIN_FILENO) };
    let mut timeout = TimeVal::new(0, 0);

    readfds.insert(stdin_fileno);

    select::select(1, &mut readfds, None, None, &mut timeout)
}
