use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};
use std::{thread, time};

fn main() {
    let seconds = 2;
    // Value literals and pointers
    let mut value = 99;
    let mut number: &i32;
    number = &-1;

    // Fork the process and panic if forking fails
    let result = unsafe{fork()}.expect("Fork failed");

    match result {
        ForkResult::Parent { child } => {
            println!("Parent value before: {}", value);
            println!("Parent number before: {}", number);
            value = 42;
            number = &9999;
            println!("Parent value after: {}", value);
            println!("Parent number after: {}", number);

            waitpid(child, None).unwrap();
        }
        ForkResult::Child => {
            println!("Child sleeping to let parent muck about") ;
            thread::sleep(time::Duration::from_secs(seconds));
            println!("Child value before: {}", value);
            println!("Child number before: {}", number);
            thread::sleep(time::Duration::from_secs(seconds));
            value = 77;
            number = &1111;

            println!("Child value after: {}", value);
            println!("Child number after: {}", number);
        }
    }
}
