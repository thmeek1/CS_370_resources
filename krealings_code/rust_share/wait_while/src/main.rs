use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};

const ITERATIONS: usize = 4;

fn main() {
    // What if you have multiple threads all waiting on a signal, but only
    // certain threads need to act once the signal comes through?

    // Create a thread-safe tuple of guarded data and a signaler
    let guarded_data = Arc::new((Mutex::new(1), Condvar::new()));

    for i in 0..ITERATIONS {
        // Clone a reference to the thread-safe data
        let cloned_ref = guarded_data.clone();

        thread::spawn(move || loop {
            // Destructure the tuple
            let (lock, cvar) = &*cloned_ref;

            // Obtain the lock
            println!("\tThread #{} waiting on lock", i);
            let val = lock.lock().unwrap();

            // Wait while the number is not the thread's ID
            // wait_while(guard, condition) blocks until the we recieve a notification on guard, and
            // condition returns false. 
            // Remember that waiting will temporarily drop the lock!

            // This is not ideal (but busy waiting is kept to a minimum)
            println!("Thread #{} attempting to modify val {}", i, val);
            let mut val = cvar.wait_while(val, |num| {
                    println!("{} is Awake, but it is {}'s turn", i, *num);
                    *num != i
                }).unwrap();

            // Decrement the data, account for rollover
            *val = if *val == 0 { ITERATIONS - 1 } else { *val - 1 };

            // Notify ALL threads waiting on the signal
            cvar.notify_all();

            println!("Thread #{} changd val to {}\n", i, val);
            thread::sleep(Duration::from_secs(1));
        });
    }

    // NOTES
    //      Notice how each thread immediately tries to reacquire the data
    //      after modifying it, but must wait until the data matches
    //      the thread's ID

    thread::park();
}
