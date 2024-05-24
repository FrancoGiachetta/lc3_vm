use std::{process, thread};

use libc::STDIN_FILENO;
use signal_hook::{consts::signal::SIGINT, iterator::Signals};
use termios::{self, tcsetattr, Termios, ECHO, ICANON, TCSANOW};

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
    let mut signals = Signals::new(&[SIGINT]).unwrap();

    thread::spawn(move || {
        for _sig in signals.forever() {
            restore_input_buffering();

            process::exit(130);
        }
    });
}
