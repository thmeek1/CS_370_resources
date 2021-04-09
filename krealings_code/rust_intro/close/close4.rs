/// All closeres implement on of the traits Fn, FnMut, or FnOnce
/// Closures can capture values from their environment in three ways, which directly map to the three ways a function
/// can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three
/// Fn traits as follows:

/// * FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume
///   the captured variables, the closure must take ownership of these variables and move them into the closure when 
///   it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same 
///   variables more than once, so it can be called only once. 
/// * FnMut can change the environment because it mutably borrows values.  
/// * Fn borrows values from the environment immutably.

fn main() {
    let answer  = foom();
    assert!(answer(99));
    assert!(answer(99));
    println!("Done");
}

fn foom() -> Box<dyn Fn(i32) -> bool> {
    let num = 99;
    Box::new(move |data| data == num)

}

