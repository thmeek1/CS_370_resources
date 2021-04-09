// While we have no threads, we can consider the 1 process we have to have 1 thread of execution. So we promise the
// rust compiler we will only use this MUTABLE static in one thread of execution.

use::std::cell::RefCell; // This is a container items where Rust wants to dynamically check borrow counts!

thread_local! {
    static VALUE: RefCell<i32> = RefCell::new(999);
}
// This locks the static RefCell with the value behind a door, so to speak -- we use with() to open the door -- for a
// scope. Once we unlock the door, we use borrow_mut() to borrow a mutable reference to the RefCell or borrow() to
// borrow an unmutable reference to the RefCell. These borrow checks are made at run time not compile time.

// In more detail. thread_local put the value inside a locked conatiner called a LocalKey. The main way to access this
// locked container is with the method with(), which wants a run-only once function passed as a parameter. with()
// can return a reference to the value within the locked container.

// In the first example we pass "|g_val| {*g_val.borrow_mut() += 1;}", which is an anonymous function, as a parameter 
// to with(). In our anonymous function g_val is the parameter that represents the RefCell, we then borrow the value
// inside the RefCell and add to it. But do not return anything from with. with() returnes references not mutable
// references.


//                 LocalKey (thread_local)
//     +----------------------------------------------------+
//     .                                                    .
//     .                                                    .
//     .               RefCell                              .
//     .    +------------------------------------------+    .
//     .    .                                          .----.---- with (function) --> reference to value inside it
//     .    .                                          .    .
//     .    .               i32------------------------.----.----borrow() --> Result(_, reference to value) 
//     .    .                                          .    .
//     .    +------------------------------------------+    .
//     .                                                    .
//     .                                                    .
//     +----------------------------------------------------+


fn main() {

    // g_val is our RefCell, borrowing from g_val gives us a mutable reference to the i32 which is mutable inside  
    // with(), which returns nothing.

    // Updating the global safely within the locked container.
    VALUE.with(|g_val| { 
        *g_val.borrow_mut() += 1;
    });

    // val is our RefCell, borrowing from val gives us an immutable reference to the i32 which we return from with()
    // This is okay because this is NOT mutable

    // Get a local copy of the global which is immutable and then printing it.
    let value_of_global = VALUE.with(|val| {
                            *val.borrow()
                          });
    println!("I can see a beautiful value with {}", value_of_global); 

    let local_value = 907;
    println!("I can see a beautiful value with {}", local_value);

    foo();

    println!("I can see a beautiful value AFTER FOO with {}", value_of_global); 
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
