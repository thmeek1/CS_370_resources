use std::sync::Arc; // Atomic Reference Counter
use std::thread;

fn main() {
    let data = 99;

    // `Arc` is a thread-safe shareable reference counter
    // We need to wrap our data in an `Arc`
    let sharable = Arc::new(data);

    // Let's clone a reference so our thread can own a reference
    // Note that this isn't cloning the data, just a reference to the data
    let reference = sharable.clone();

    let child = thread::spawn(move || {
        *reference = 109; // Cannot modify, since this shared memory and not safe
                          // we will fix this in arc_mutex_simple
        println!("Number is {}", reference);
    });

    child.join().unwrap();

    println!("A number :{}", data);
}
