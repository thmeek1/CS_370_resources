use std::sync::Arc; // Atomic Reference Counter
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn main() {
    let data = AtomicI32::new(99);

    // `Arc` is a thread-safe shareable reference counter
    // We need to wrap our data in an `Arc`
    let sharable = Arc::new(data);
    println!("A number :{:?}", *sharable);

    // Let's clone a reference so our thread can own a reference
    // Note that this isn't cloning the data, just a reference to the data
    let reference = sharable.clone();

    let child = thread::spawn(move || {
        (*reference).store(109, Ordering::Relaxed);
        println!("Number is: {:?}", reference);
    });

    child.join().unwrap();

    println!("A number :{:?}", *sharable);
}
