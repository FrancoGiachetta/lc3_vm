pub fn halt(running: &mut bool) {
    println!("HALT");
    *running = false;
}
