use std::sync::mpsc::channel;
use std::thread;
// multiple senders

fn main() {
    println!("\nSending and receiving messages using multiple senders:");

    let values = vec!["Let", "there", "be", "pizza", "YUM"];

    // You can also just annotate the channel
    let (sender, receiver) = channel::<&str>();

    for val in values {
        let cloned_sender = sender.clone();
        // Spin up a new thread to send messages from
        thread::spawn(move || {
            cloned_sender.send(val).unwrap();
        }); // drops cloned_sender because the function is done
    }
    // We don't know how long the `sender` clones will live
    // We must explicitly drop the original `sender` (the one we cloned everyone from)  so that the `receiver` knows
    // that no more messages will be sent

    drop(sender); // Comment me out and see what happens!


    // Note that this will block until .... all senders are closed/dropped
    println!("Alert:");
    for received in receiver {  // iterator that calls recv() for use 
        println!("{}", received);
    }
}
