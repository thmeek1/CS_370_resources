use std::sync::Mutex;

fn main() {
    let data = "Get a Math minor!";

    // `Mutex` is a singly-accessible data guard (AKA binary semaphore)
    let guarded = Mutex::new(data);
    println!("The original Mutex:\n{:?}\n", guarded);

    // Notice I am starting a block which means new lifetime and scope 
    {
        // In order to modify the protected data, we need to lock it so that
        // we can ensure only this thread has access to it.
        let mut guarded_inner_data = guarded.lock().unwrap(); // this actually returns a MutexGuard

        println!("The inner guard:\n{:?}\n", guarded_inner_data);

        // We also need to dereference the guard to modify the inner data
        // You can use Deref or DerefMut to access the data  (*x) is shortcut 
        *guarded_inner_data = "Become a double major!";  

        println!("The locked Mutex:\n{:?}\n", guarded);
    } // The lock is dropped after it goes out of scope

    println!("The lock was dropped:\n{:?}\n", guarded);
}
