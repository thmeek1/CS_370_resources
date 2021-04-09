use std::thread;

fn main() {
    println!("Hello from the main thread!");

    // We are obtaining a "handle" to the child thread
    let child = thread::spawn(|| {
        println!("Hello from the child!");
    });

    // We are "joining" the handle, telling the main thread to stop executing
    // until the child is done
    child.join().unwrap();

    println!("Child thread is done! Exiting");
}
