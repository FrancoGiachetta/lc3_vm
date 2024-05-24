use std::io::{stdin, Read};

pub fn get_char() -> u8 {
    let mut buffer = [0_u8];
    
    stdin().read_exact(&mut buffer).unwrap();

    buffer[0]
}
