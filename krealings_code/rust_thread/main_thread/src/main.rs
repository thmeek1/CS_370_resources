use std::thread;
use std::time;

const NUM_WORKERS: usize = 5;

// The names 'MIDDLE' and 'WORKER' are made up to distinguish threads and have no special connotation outside of this
// example. 

// The MIDDLE thread is a child of the MAIN thread.
// The WORKER threads are children of the MIDDLE thread.

fn main() {
    // While I am the main thread I am also a process, in my own right.
    // However, a thread id is not a process id
    println!("Hello! I am the MAIN thread {:?}", thread::current().id());

    // Create a MIDDLE thread who will then create other threads
    let worker = thread::spawn(|| {
        thread::sleep(time::Duration::from_secs(2));
        // I am a lightweight process
        println!("Hello! I am the MIDDLE thread {:?}", thread::current().id());
        do_work();
    });

    // Main thread waits on the MIDDLE thread
    //worker.join().unwrap();

    // Stop the main thread from terminating
    thread::park();
    //thread::sleep(time::Duration::from_secs(3));
    println!("Now the MAIN thread is done");
}

fn do_work() {
    // Storage for the handles for our child threads
    let mut workers = Vec::with_capacity(NUM_WORKERS);

    // Creating a number of threads in sequence (also lightweight processes)
    for _ in 0..NUM_WORKERS {
        let child = thread::spawn(|| {
            thread::sleep(time::Duration::from_secs(5));
            println!("Hello! I am a WORKER thread{:?}", thread::current().id());
        });

        workers.push(child);
    }

    //for worker in workers {
    //    worker.join().unwrap();
    //}

    //The MIDDLE or parent of the workers terminates, without waiting
    println!("Now the MIDDLE thread is done");
}
