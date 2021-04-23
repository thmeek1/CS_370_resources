use std::sync::Arc; // Atomic Reference Counter
use std::thread;

fn main() {
    let data = "Get a Math minor!";

    // `Arc` is a thread-safe shareable reference counter
    // We need to wrap our data in an `Arc`
    let sharable = Arc::new(data);

    // Let's clone a reference so our thread can own a reference
    // Note that this isn't cloning the data, just a reference to the data
    let reference = sharable.clone();

    let child = thread::spawn(move || {
        println!("Your daily fortune is '{}'", reference);
    });

    child.join().unwrap();

    println!("I hope you follow the great advice: {}", data);
}
