use std::thread;
use std::time;

const NUM_CHILDREN: usize = 5;

fn main() {
    println!("Let's make some new threads");

    // Create a number of children,
    // Pre-allocate enough space when we make our vector
    let mut children = Vec::with_capacity(NUM_CHILDREN);

    // Creating a number of threads in sequence
    for _ in 0..NUM_CHILDREN {
        let child = thread::spawn(|| {
            thread::sleep(time::Duration::from_secs(10));
            println!("Hello! I am a child {:?}", thread::current().id());
        });

        children.push(child);
    }

    // When will this message appear?
    println!("Let's collect the threads");

    // Joining the threads in sequence
    for child in children {
        child.join().unwrap();
    }

    println!("Now the main thread is done");
}
