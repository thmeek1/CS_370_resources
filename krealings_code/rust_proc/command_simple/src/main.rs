use std::process::Command;

fn main() {
    println!("Lets make our child process do some work!");

    // We are spawning a new command and letting it run independently
    // in its own child Process
    let process_name = "ls";

    println!("\nExecuting `{}` command\n", process_name);

    // Command::new creates a structure and then spawn() creates the new process with the image set up in the
    // structure
    Command::new(process_name).spawn().expect("Failed to launch new process");
}
