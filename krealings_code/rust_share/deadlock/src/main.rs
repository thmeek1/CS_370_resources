use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    // Let's simulate a deadlock
    // Two threads will attempt to access the same data without properly
    // handling the locks
    let guarded_data = Arc::new((Mutex::new(false), Condvar::new()));

    for i in 0..2 {
        // Cloning a REFERENCE to the `ARC`
        let cloned_ref = guarded_data.clone();

        thread::spawn(move || {
            // Breaking apart the tuple
            let (lock, cvar) = &*cloned_ref;
            println!("In the thread #{}", i);

            // Obtaining the lock
            let val = lock.lock().unwrap();

            // This thread is waiting to be notified
            let val = cvar.wait(val).unwrap();
            println!("End of thread #{} with {}", i, val);
        });
    }

    // QUESTIONS:
    //      What's causing this to deadlock?
    //      What could you add/delete/rewrite to fix it?
    //      How could you rewrite this?

    // Parking the main thread so it will not finish before all child threads
    thread::park();
}
