// While we have no threads, we can consider the 1 process we have to have 1 thread of execution. So we promise the
// rust compiler we will only use this MUTABLE static in one thread of execution.

use::std::cell::RefCell; // This is a container items where Rust wants to dynamically check borrow counts!

thread_local! {
    static VALUE: RefCell<i32> = RefCell::new(999);
}
// This locks the static RefCell with the value behind a door, so to speak -- we use with() to open the door -- for a
// scope Once we unlock the door, we use borrow_mut() to borrow a mutable reference to the RefCell or borrow() to
// borrow an unmutable reference to the RefCell. These borrow checks are made at run time not compile time.


fn main() {

    VALUE.with(|g_val| { 
        *g_val.borrow_mut() += 1;
    });
    println!("I can see a beautiful value with {}", 
        VALUE.with(|val| {
            *val.borrow()
        })
    );
    let local_value = 907;
    println!("I can see a beautiful value with {}", local_value);

    foo()
}

fn foo() {
    println!("FOO can see a beautiful value with {}", 
        VALUE.with(|val| {
            *val.borrow()
        })
    );
    let local_value = 807;
    println!("FOO can see a beautiful value with {}", local_value);

}
