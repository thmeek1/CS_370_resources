// The nix crates map to UNIX standard functions

use nix::unistd::{fork, getpid, ForkResult, sleep};

fn main() {
    // fork returns a Result<ForkResult>
    // Currently one proc (Parent)
    let result = unsafe{fork()}; // call once returns twice

    // inow two procs -- Parent and Child
    let current_process_id = getpid();

    // Match what could have been returned from fork()
    match result {
        Ok(ForkResult::Parent { child }) => {
            println!("In the Parent process with pid {}, new child has pid {}", current_process_id, child);
        }

        Ok(ForkResult::Child) => println!("In the Child process with pid {}", current_process_id),

        Err(e) => println!("Fork failed\n{}", e),
    }
}
