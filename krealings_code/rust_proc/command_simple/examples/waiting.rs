use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Lets make our child process do some work!");
    thread::sleep(Duration::from_millis(100));

    // We can immediately collect a process' output
    let process_name = "ls";
    let args = ["-a", "-t"];

    println!("\nExecuting `{}` command, waiting on output\n\n", process_name);

    let output = Command::new(process_name)
        .args(&args)
        .output() // wait to finish and collect ALL the processes output (this is important)
        .expect("Failed to launch new process");

    println!("\n\nNew process output is: {:#?}\n", output);
}
