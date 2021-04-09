/*
 * All four are legal and equal
 *
 * (A) fn  functionName    (data: type) -> type { operations }
 * (B) let closureName   = |data: type| -> type { operations };
 * (C) let closureName   = |data|               { operations };
 * (D) let closureName   = |data|                 operations;
 */

fn main() {
    // (D) A closure is an anonymous function
    let two_times = |data| data * 2;

    // You call it like a normal function
    println!("Using closure: {}\n", two_times(2));

    // (C)You can have multiple parameters
    let add_cl = |one, two| {one + two};

    println!("adding 5 and 8 via closure: {}", add_cl(5, 8));
    println!("adding 6 12 via function: {}\n", add_fn(6, 12)); // defined below

    // Maybe you do not need any parameters
    let get_word = || "hello";

    println!("Word is: {}\n", get_word());

    // Closures can even be entire expressions
    let big_closure = || {
        for i in 1..101 {
            if i % 33 == 0 {
                println!("Here's a third: {}", i);
            }
        }
    };

    big_closure();
}

/// Here's a function identical to the closure:
/// let add_cl = |one, two| one + two;
fn add_fn(one: i32, two: i32) -> i32 {
    one + two
}
