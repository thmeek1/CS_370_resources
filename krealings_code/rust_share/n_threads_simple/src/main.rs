use std::sync::{Arc, Mutex};
use std::{thread, time};

const NUM_CHILDREN: i32 = 3;
const ITERATIONS: i32 = 4;

fn main() {
    println!("Simulating {} threads sharing a lock\n", NUM_CHILDREN);

    // Protected shared data and a conditional variable notifier
    let guarded_data = Arc::new(Mutex::new(5));

    for i in 0..NUM_CHILDREN {
        // Get a reference to the shared data (in this case a Mutex)
        let cloned_ref = guarded_data.clone();

        thread::spawn(move || {
            println!("Spinning up child thread #{}", i);
            
            loop {
                // If another thread holds the lock, this thread will go to
                // sleep until the lock is made available
                println!("Child #{} is attempting to obtain the locked data", i);
                let mut value = cloned_ref.lock().unwrap();
                println!("Child #{} got the data.", i);

                // Do some work to the value within the lock
                for _ in 0..ITERATIONS {
                    thread::sleep(time::Duration::from_secs(1));
                    *value = if i % 2 == 1 { *value + 1 } else { *value - 1 };
                    println!("In child #{} value is now {}", i, value);
                }

                println!("Child #{} is now done with the data\n", i);

                // Wait a brief moment to ensure that this thread does not
                // immediately re-obtain the lock
                // NOTE: Try removing this wait and see what happens
                thread::sleep(time::Duration::from_millis(25));
            }
        });
    }

    println!("Pausing parent runtime\n");
    thread::park();
}
