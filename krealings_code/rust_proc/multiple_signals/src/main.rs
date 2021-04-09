use addy::{mediate, Signal, SIGINT, SIGWINCH};
use std::process::exit;
use std::{thread::sleep, time::Duration};

fn main() {
    // Here we are registering a handler function to handle `SIGWINCH`
    mediate(SIGWINCH)
        .register("Window Resize Handler", sigwinch_handler) // Registering the handler
        .expect("Failed to register signal handler")
        .enable() // Enabling the handler
        .expect("Failed to enable signal handler");

    mediate(SIGINT)
        .register("Interrupt Handler", sigint_handler) // Registering the handler
        .expect("Failed to register signal handler")
        .enable() // Enabling the handler
        .expect("Failed to enable signal handler");

    loop {
        println!("Doing arbitrary work! Resize window or send SIGINT");
        sleep(Duration::from_secs(3));
    }
}

/// Handles `SIGWINCH`
fn sigwinch_handler(signal: Signal) {
    println!("Received {}", signal);
}

/// Handles `SIGINT`
fn sigint_handler(signal: Signal) {
    println!("\n{} caught! Shutting down", signal);
    sleep(Duration::from_secs(1));
    exit(1);
}
