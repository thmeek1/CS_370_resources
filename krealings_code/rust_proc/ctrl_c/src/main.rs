use ctrlc::set_handler; // Be sure to include this crate in `[dependencies]`
use std::process::{exit, id};
use std::{thread::sleep, time::Duration};

fn main() {
    // We are assigning a function to execute whenever SIGINT is caught
    set_handler(handler).expect("Failed to set signal handler");

    // Now we'll just loop infinitely until the user press `CTRL+C`
    loop {
        // We're also printing the process ID so we can explicitly send signals
        println!("({}) Waiting to receive CTRL+C / SIGINT...", id());
        sleep(Duration::from_secs(1));
    }
}

/// Handles `SIGINT`
fn handler() {
    println!("\nSignal caught! Exiting now");
    exit(1);
}
