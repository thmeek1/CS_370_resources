use nix::unistd::{fork, ForkResult};
use std::{process, thread, time};

// Where is the zombie in this program? What creates it?
fn main() {
    let result = fork().expect("Failed to fork");

    match result {
        ForkResult::Parent { child: _ } => loop {
            thread::sleep(time::Duration::from_secs(1000));
        },
        ForkResult::Child => {
            process::exit(0);
        }
    }
}
