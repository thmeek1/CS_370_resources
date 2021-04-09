// Used to get user input
use std::io::stdin;

// Used to flush standard output
use std::io::{stdout, Write};

fn main() {
    basic();

    prompt();

    get_num();
}

/// Basic user input
fn basic() {
    // To obtain user input, we need to allocate space for the input first
    let mut buffer = String::new();

    // Now we are locking `stdin` and rerouting its input to a variable
    stdin().read_line(&mut buffer).expect("Failed to read user input");

    println!("Input: {}", buffer);
}

/// User input with a prompt
fn prompt() {
    // Here we are prompting for user input and immediately flushing our
    // prompt to the screen
    print!("Please input a string > ");
    stdout().flush().expect("Failed to flush stdout");

    // To obtain user input, we need to allocate space for the input first
    let mut buffer = String::new();

    // Now we are locking `stdin` and rerouting its input to a variable
    stdin().read_line(&mut buffer).expect("Failed to read user input");

    println!("Input: {}", buffer);
}

/// How to parse input to be a number
fn get_num() {
    // Here we are prompting for user input and immediately flushing our
    // prompt to the screen
    print!("Please input a number > ");
    stdout().flush().expect("Failed to flush stdout");

    // To obtain user input, we need to allocate space for the input first
    let mut buffer = String::new();

    // Now we are locking `stdin` and rerouting its input to a variable
    stdin().read_line(&mut buffer).expect("Failed to read user input");

    // Trim the leading and trailing whitespace, then parse it explicitly as
    // an i32, panicking if that fails
    let parsed = buffer.trim().parse::<i32>().expect("Failed to parse input");

    println!("Parsed: {}", parsed);
}
