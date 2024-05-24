use byteorder::{BigEndian, ReadBytesExt};
use std::{
    fs::File,
    io::{self, BufReader, ErrorKind},
};

use crate::memory::Memory;

pub fn read_image_file(file: File) -> io::Result<Memory> {
    let mut buffer = BufReader::new(file);
    let origin = buffer.read_u16::<BigEndian>()?;
    let mut address = origin;
    let mut mem = Memory::new();

    loop {
        match buffer.read_u16::<BigEndian>() {
            Ok(inst) => {
                mem.mem_write(address, inst);
                address += 1;
            }
            Err(e) => {
                return if e.kind() == ErrorKind::UnexpectedEof {
                    Ok(mem)
                } else {
                    Err(e)
                }
            }
        }
    }
}

pub fn read_image(name: String) -> io::Result<Memory> {
    let f = File::open(name)?;
    read_image_file(f)
}
