use nix::sys::wait::waitpid;
use nix::unistd::{execve, fork, getpid, ForkResult};
use std::ffi::CString;

fn main() {
    //fork_exec
    // We are setting up a mock program with arguments and environment variable
    let program_name = CString::new("example_program").expect("CString creation failed");
    let arg1 = CString::new("arg1").expect("CString creation failed");
    let arg2 = CString::new("arg2").expect("CString creation failed");
    let args = &[arg1, arg2];
    let env = &[CString::new("env=val").expect("CString creation failed")];

    for _ in 0..2 {
        // Fork the process, obtaining which process is current and the ID
        let result = unsafe{fork()};
        let current_process_id = getpid();

        match result {
            Ok(ForkResult::Parent { child }) => {
                println!(
                    "I am the Parent ({}), waiting for the Child ({})",
                    current_process_id, child
                );
                // Wait for the child to complete
                let status = waitpid(child, None);
                println!(
                    "Child {} has finished with a status of {:?}\n",
                    child, status
                );
            }
            Ok(ForkResult::Child) => {
                println!("I am the Child ({})", current_process_id);
                // Pass another program to the child process
                execve(&program_name, args, env).unwrap();
            }
            Err(e) => println!("Fork failed\n{}", e),
        }
    }
}
