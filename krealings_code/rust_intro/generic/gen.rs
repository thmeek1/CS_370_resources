use std::fmt::Display; // Bring into scope

// Typical unbounded generic
#[derive(Debug)] // needed to display struct
struct GenericData<T, E> {
    data: T,
    other: E,
}

fn main() {
    let x = GenericData {
        data: 3,
        other: false,
    };
    let y = GenericData {
        data: "Hello",
        other: 4.0,
    };

    println!("Some generic struct: {:?}", x);
    println!("Some generic struct: {:?}", y);

    generic_func(x.other);
    generic_func(y.data);
}

// We are accepting a generic parameter and bounding it
// This ensures that this function can ONLY accept generics that implement
// the `Display` trait and are printable
fn generic_func<T: Display>(data: T) {
    println!("Received generic data: {}", data);
}
