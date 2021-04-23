use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;



const ITERATIONS: usize = 10;

fn main() {
    let data = 2;

    println!("Original value: {}", data);

    // Wrap the data in a `Mutex` AND an `Arc`
    let guarded_ref = Arc::new(Mutex::new(data));

    // Create a vector to hold a few threads
    let mut children = Vec::with_capacity(ITERATIONS);

    for i in 0..ITERATIONS {
        // Clone a reference to the guarded data
        let cloned_ref = guarded_ref.clone();

        // Move data into a child thread
        let child = thread::spawn(move || {
            // Obtain the lock
            if i == 2 {
                thread::sleep(Duration::from_secs(2));
            }
            let mut num = cloned_ref.lock().unwrap(); // remember num is a mutex guard not 'just' the data

            println!("{} Doubling...", i);
            // Dereference and double the interior data
            if i == 6 {
                thread::sleep(Duration::from_secs(4));
            }
            *num *= 2;
        });

        children.push(child);
    }

    // wait for all child threads to stop
    for child in children {
        child.join().unwrap();
    }

    // Obtain the lock and print the final value
    let final_val = guarded_ref.lock().unwrap();
    println!("Final value: {}", *final_val);
}
