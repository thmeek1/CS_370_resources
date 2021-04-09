use std::process::Command;
use nix::unistd::{getpid, getppid};
use std::thread;
use std::time::Duration;

fn main() {


    let child = Command::new("child").spawn().expect("Failed to launch new process");

    loop {
        println!("parent {} --> my parent is {} (bash)", getpid(), getppid());
        println!("\nmy child is {}", child.id());
        thread::sleep(Duration::from_millis(500));
    }
}
