use std::os::raw::c_void;

use libc::{c_int, sighandler_t};

use crate::setup::restore_input_buffering;

extern "C" {
    fn getchar() -> c_int;
}

pub fn get_char() -> i32 {
    unsafe { getchar() }
}

extern "C" fn handle_interrupt() {
    restore_input_buffering();
    println!("\n");
    std::process::exit(-2);
}

pub unsafe fn singnal_int() -> sighandler_t {
    handle_interrupt as extern "C" fn() as *mut c_void as sighandler_t
}
