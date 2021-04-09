// You can import an entire file
pub mod utils;

// Or just specific functions/structs/etc
pub mod functions;
use crate::functions::func;

// Here we are calling a function that calls another function from another file
pub mod calling_eachother;
use crate::calling_eachother::calls_another;

// For importing from lib.rs, you use the crate name
use multiple_files::{another_module, some_module};

fn main() {
    // We must use module qualifiers ( `::` ) because we imported the entire `utils` module
    let _struct = utils::Thing { data: 10 };

    // Calling from `functions.rs`
    func();

    // Calling from `calling_eachother.rs` which calls from `functions.rs`
    calls_another();

    // Using `lib.rs`, which is idiomatic
    some_module::some_function();

    // Using `lib.rs` to display private data
    another_module::print_new_struct();
}
