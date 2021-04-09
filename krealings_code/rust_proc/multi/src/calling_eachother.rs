// We need to import the other function from the current crate
use crate::functions::func;

pub fn calls_another() {
    println!("We're going to call a function from another file now!");
    func();
}
