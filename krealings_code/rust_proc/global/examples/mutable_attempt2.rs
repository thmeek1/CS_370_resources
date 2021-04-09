// Lets wrap the Static variable in a container that can count the references so Rust can check safety. Hoorah!

use::std::cell::RefCell; // This is a container items where Rust wants to dynamically check borrow counts!

static VALUE: RefCell<i32> = RefCell::new(999);
// We use borrow_mut() to borrow a mutable reference to the RefCell or 
//        borrow()     to borrow an immutable reference to the RefCell


fn main() {

    *VALUE.borrow_mut() += 1;
    println!("I can see a beautiful value with {}", *VALUE.borrow());
    let local_value = 907;
    println!("I can see a beautiful value with {}", local_value);

    foo();
}

fn foo() {
    println!("FOO can see a beautiful value with {}", *VALUE.borrow());
    let local_value = 807;
    println!("FOO can see a beautiful value with {}", local_value);

}
