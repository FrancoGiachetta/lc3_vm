use std::io::{stdout, Write};

pub fn halt(running: &mut bool) {
    println!("HALT");
    stdout().flush().expect("Flushed");

    *running = false;
}
