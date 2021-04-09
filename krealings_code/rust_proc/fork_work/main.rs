use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, ForkResult, sleep};

//fork_work
fn main() {
    let seconds = 3;

    for _ in 0..2 {
        let result = unsafe{fork()};
        let current_process_id = getpid();

        match result {
            Ok(ForkResult::Parent { child }) => {
                println!("I am the Parent, waiting for Child ({})", child);
                let status = waitpid(child, None);
                println!(
                    "Child ({}) has finished with status {:?}",
                    child,
                    status.unwrap()
                );
            }
            Ok(ForkResult::Child) => {
                println!(
                    "I am the Child ({}) and I will sleep for {} seconds",
                    current_process_id, seconds
                );
                sleep(seconds);
                break;
            }
            Err(e) => println!("Fork failed\n{}", e),
        }
    }
}
