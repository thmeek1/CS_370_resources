use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Lets make our child process do some work!");
    thread::sleep(Duration::from_millis(100));

    // We can also spawn a process, do something else, then wait on output
    let process_name = "ls";
    let args = ["-a", "-t"];

    println!("\nExecuting `{}` command\n\n", process_name);

    // Command::New() creates a Command Structure that *will* have the process image defined as the argument
    // when we spawn() it (create it and run it)
    // .args() sets up the arguments it will get when we create and run it
    // spawn() creates the process and runs it. spawn() returns a Child Structure (or an error) which is a handle
    // to the newly created process.
    let process = Command::new(process_name) // process is a Child struct
        .args(&args)
        .spawn() // return a handle, but do not wait for process to finish
        .expect("Failed to launch new process");

    for i in 1..1000 {
        if i % 333 == 0 {
            println!("I'm just here to simulate work");
        }
    }
    // Here we are waiting on the process.
    let output = process.wait_with_output().expect("Failed to wait");

    println!("\n\nNew process exited with status: {:?}\n", output.status);
}

