// There are two type of strings in Rust
// 'str' AKA a string slice (A slice of a string), it is usually seen in its borrowed for '&str'
// str is the most primitive type of string; string literals are string slices.
// str is fixed length, immutable, located somewhere in memory
// String slices are always valid UTF-8 (growable, heap based)

// The other type is String (StringBuffer in Java)


fn main() {
    
    let _hello = "Hello world"; // I am a string slice, I live on the stack (most likely), Cool huh!
                                // I am by default static and live forever, MUH AH HA HA
                                // Okay for the life of this program at least

    let _hi: &'static str = "Hi Mom"; // Basically same as above, but more explicit! (in static memory)

    // yet another string slice. Whay do I always want pizza after programming in Rust?
    let dinner: &str = "Pizza is tasty";

    // The `str` type is stack-allocated (usually) and therefore its size must be
    // known at compile time. Hence why it is often "sliced" with &

    // A `String` is an owned, heap-allocated, growable string
    let mut heap_string: String = String::new();
    heap_string.push('I');
    heap_string.push_str(" am cool");

    // Can also be created from a literal
    let mut cats = String::from("Go Cats!");

    // Can also be created from a literal, part Duex
    let mut cheese = "chedder".to_string();

    // A `String` can be referenced via slice (`&str`) but not vice versa
    take_string(cats);
    take_string(cheese);

    // NOPE
    //take_string(dinner);

    cats = String::from("Go Cats!");    // reusing the variables, so they were made mutable earlier
    cheese = "blue".to_string();

    take_both(&cats); // &cats points to the heap, not the stack
    take_both(&cheese);
    take_both(&dinner);
}

// Functions that can only accept a `String`
fn take_string(data: String) {
   println!("I got a a full String: {}", data);
}

// Can accept from either a `str` or a `String`
fn take_both(data: &str) {
   println!("I got a slice: {}", data);
}
