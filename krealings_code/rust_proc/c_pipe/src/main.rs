use nix::unistd::{sleep};
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    let seconds = 3;
    let process  = match Command::new("r_pipe")
                                  .stdin(Stdio::piped())
                                  .spawn() {
        Err(why) => panic!("Could not spawn program {}", why),
        Ok(process) => process,
    };

    let buffer = "Super secret Data".as_bytes();
    sleep(seconds);
    process.stdin.unwrap().write_all(buffer).expect("Failed to write");
}
