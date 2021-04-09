use std::io::{self, Write, Read};

const BUF_SIZE: usize = 1024;

fn main() {
   
    let mut buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];
    io::stdout().write_all(b"print buffer\n").expect("Failed to write");
    io::stdout().write_all(&buffer).expect("Failed to write");
    io::stdout().write_all(b"Now read from stdin\n").expect("Failed to write");
    io::stdin().read(&mut buffer).expect("Failed to read");
    io::stdout().write_all(&buffer).expect("Failed to write");
    io::stdout().write_all(b"\n").expect("Failed to write");
    io::stdout().write_all(b"CHILD PROC DONE\n").expect("Failed to write");
}
