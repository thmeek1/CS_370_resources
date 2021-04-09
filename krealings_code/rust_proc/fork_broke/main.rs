use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, ForkResult, sleep};

//fork_broke
fn main() {
    for _ in 0..2 {
        // Fork the process, obtaining which process is current and the ID
        let result = unsafe{fork()};
        let current_process_id = getpid();

        match result {
            Ok(ForkResult::Parent { child }) => {
                println!(
                    "My result is {:?} thus I am a Parent ({})",
                    result.unwrap(),
                    current_process_id
                );

                // We are causing the parent thread to wait for itself to change state
                let status = waitpid(current_process_id, None);

                println!(
                    "Child ({}), has finished with a status of {:?}\n",
                    child, status
                );
            }
            Ok(ForkResult::Child) => {
                println!(
                    "My result is {:?} thus I am a Child ({})\n",
                    result.unwrap(),
                    current_process_id
                );

                // Simulate child process runtime by waiting
                sleep(3);
            }
            Err(e) => println!("Fork failed\n{}", e),
        }
    }
}
