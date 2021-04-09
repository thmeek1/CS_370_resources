use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Lets make our child process do some work!");
    thread::sleep(Duration::from_millis(100));

    // We can also spawn a process, do something else, then wait on output
    let process_name = "example_program";
    let args = ["arg1", "catDog"];

    println!("\nExecuting `{}` command", process_name);

    let status = Command::new(process_name)
        .args(&args)
        .status() // waits for child to complete and collects status
        .expect("Failed to launch new process");
    thread::sleep(Duration::from_millis(100));

    println!("\n\nNew process exited with status: {:?}\n", status);
}

