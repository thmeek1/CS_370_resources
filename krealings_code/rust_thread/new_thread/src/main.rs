use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread says hello!");

    // Spawn detached the child thread. In this case the child *MAY* outlive the parent, UNLESS
    // the parent is the main thread; the whole process is terminated when the main thread finished.
    thread::spawn(|| {
        println!("Child thread says hello!");
    });

    // Try uncommenting this to see the effect, is this guarantueed?
    //thread::sleep(Duration::from_millis(100));

    println!("Main thread after child has been declared");

}
