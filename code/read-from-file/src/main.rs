use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("file.txt")?;
    let mut buffer: [u8; 1024] = [0; 1024];

    // read a portion of the file to the buffer
    let bytes_read = file.read(&mut buffer)?;

    // process the buffer as a slice
    let slice = &buffer[0..bytes_read];
    println!("Read {} bytes: {:?}", bytes_read, slice);

    Ok(())
}