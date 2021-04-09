// It is common to have modules grouped in lib.rs
pub mod some_module {
    pub fn some_function() {
        println!("In my own module, in lib.rs!");
    }
}

// This module access its own private data through a public function
pub mod another_module {
    #[derive(Debug)]
    struct Printable;

    pub fn print_new_struct() {
        println!("Here's a struct: {:?}", Printable);
    }
}
