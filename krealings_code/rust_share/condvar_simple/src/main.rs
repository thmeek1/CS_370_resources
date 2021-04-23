use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time};

fn main() {
    // Note that this `Arc` contains a tuple of a `Mutex` AND a `Condvar`
    let guarded_data = Arc::new((Mutex::new(5), Condvar::new()));

    let cloned_ref = guarded_data.clone();

// ***************THREAD CODE
    thread::spawn(move || {
        // Splitting up the tuple to obtain the `Mutex` and `Condvar`
        // Here we have to get a reference to the data inside, a reference to the data, which must be dereferenced

        let (lock, cvar) = &*cloned_ref;  // this assignment borrows the values in the tuple
        //let (lock, cvar) = *cloned_ref; // this assignment transfers ownership of the values in the tuple, 
                                          // which is not allowed for an Arc
        //let (lock, cvar) = cloned_ref;  // this assigment is the tuple itself, NOT the tuple's data

        // Now we're just back to modifying the data guarded by the mutex
        let mut data = lock.lock().unwrap();

        // Modify the data over time
        while *data < 10 {
            thread::sleep(time::Duration::from_secs(1));
            *data += 1;
            println!("Incrementing, data is now {}", data);
        }
        // Data has been modified. Alert a thread waiting, which is just main, in this example
        cvar.notify_one();
    });
// ***************THREAD CODE END

    
    // Splitting up the tuple to get the data within
    let (lock, cvar) = &*guarded_data;
    let mut data = lock.lock().unwrap(); // LINE A lock the data
    println!("Before waiting, data is {}", data);

    // Wait until data has been updated (another process calls notify_one() on the condition variable)
    // Note that this will block the thread entirely
    //      The Mutex 'data' held by main (from line A) is released on the wait()
    //      No busy waiting, the thread is entirely asleep until `cvar` is used to signal main
    println!("Main thread will now sleep until notified");
    data = cvar.wait(data).unwrap();
    // At this point the lock is reaquired
    println!("After waiting, lock is {:?}", lock);
    println!("After waiting, data is {}", data);
}
