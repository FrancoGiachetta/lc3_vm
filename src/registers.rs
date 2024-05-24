use std::ptr::null_mut;

use libc::{FD_SET, FD_ZERO, STDIN_FILENO};

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

pub fn check_key() -> bool {
    let mut readfds: libc::fd_set = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    unsafe {
        FD_ZERO(&mut readfds);
        FD_SET(STDIN_FILENO, &mut readfds);

        let writefds = null_mut();
        let errorfds = null_mut();
        let mut timeout: libc::timeval = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };

        libc::select(1, &mut readfds, writefds, errorfds, &mut timeout) != 0
    }
}
