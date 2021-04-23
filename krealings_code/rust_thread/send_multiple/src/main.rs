use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
//send multiple (2)

fn main() {
    println!("\nSending and receiving messages:");

    let values = vec!["WCU", "GO", "CATS", "2021", "Beez"];

    // You can annotate senders and receivers if you'd like
    let (sender, receiver): (Sender<&str>, Receiver<&str>) = channel();

    // Spin up a new thread to send messages from
    thread::spawn(move || {
        for val in values {
            sender.send(val).unwrap();
        }
    });

    // We can iterate over all messages received
    // This also blocks the current thread until no more messages are left
    println!("Alert:");
    for received in receiver {
        println!("{}", received);
    }
}
