use std::sync::mpsc::channel;
use std::thread;
//send one

fn main() {
    println!("\nSending and receiving a single message:");

    // We are declaring a sender and receiver
    let (sender, receiver) = channel();

    // Spin up a new thread and send a message from it
    thread::spawn(move || {
        // Note that the sender has been moved into the thread!
        sender.send("Stay alive!").unwrap();
    });

    // `recv()` blocks the current thread until a message arrives
    // The message will either be the data sent or an error
    let received = receiver.recv().unwrap();
    println!("Transmission received: {}\n", received);
}
