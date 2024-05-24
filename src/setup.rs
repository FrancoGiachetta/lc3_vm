use libc::{SIGINT, STDIN_FILENO};
use termios::{self, tcsetattr, Termios, ECHO, ICANON, TCSANOW};

use crate::utils::singnal_int;

pub fn disable_input_buffering() {
    let mut termios = Termios::from_fd(STDIN_FILENO).unwrap();

    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &termios).unwrap();
}

pub fn restore_input_buffering() {
    let mut term: Termios = Termios::from_fd(STDIN_FILENO).unwrap();

    //turn on canonical mode and echo mode
    term.c_lflag |= ICANON | ECHO;
    tcsetattr(STDIN_FILENO, TCSANOW, &term).unwrap();
}

pub fn setup() {
    unsafe { libc::signal(SIGINT, singnal_int()) };
    disable_input_buffering();
}
