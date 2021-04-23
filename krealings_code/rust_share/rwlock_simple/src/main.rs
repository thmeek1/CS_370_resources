use std::sync::RwLock;

fn main() {
    // Very similar to a `Mutex`
    let rw_locked_data = RwLock::new(10);

    // Like a `Mutex`, we can edit the inner data via a single writer
    let mut writer = rw_locked_data.write().unwrap();
    *writer *= 2;

    println!("After writing: {}", *writer);
    drop(writer); // runs a destructor on this item (needs drop trait)
                  // this typically happens when droppable items go out of scope

    {
        // Unlike a `Mutex`, we can read many times
        let reader1 = rw_locked_data.read().unwrap();
        let reader2 = rw_locked_data.read().unwrap();
        let reader3 = rw_locked_data.read().unwrap();

        println!("{} == {} == {}", reader1, reader2, reader3);
    } // All readers are dropped at the end of the scope
}
