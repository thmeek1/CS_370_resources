use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Lets make our child process do some work!");
    thread::sleep(Duration::from_millis(100));

    // We can also spawn a process, do something else, then wait on output
    let process_name = "ls";
    let args = ["-a", "-t"];

    println!("\nExecuting `{}` command", process_name);

    let status = Command::new(process_name)
        .args(&args)
        .status() // wait for the process to finish and collect the process's status
        .expect("Failed to launch new process");
    thread::sleep(Duration::from_millis(100));
    println!("\n\n");

    for i in 1..1000 {
        if i % 333 == 0 {
            println!("I'm just here to simulate work");
        }
    }

    println!("\n\nNew process exited with status: {:?}\n", status);
}

