use nix::unistd::{getppid, getpid};
use std::thread;
use std::time::Duration;
fn main() {
    loop {
        println!("I am a child process with id {:?} -> parent({:?})", getpid(), getppid());
        thread::sleep(Duration::from_millis(500));
    }
}
