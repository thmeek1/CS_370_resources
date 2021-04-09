fn main() {
    // Many functions return an `Result` enum, which is either
    //      Ok(some value)
    //      Err(some error)
    // There are plenty of ways to "unwrap" and obtain the inner data

    ok_example();
    err_example();
    unwrap_example();
    //panic_example(); // Will panic
    //expect_example(); // Will panic
    match_example();
    if_let_example();
}

/// Ok(value) is a successful `Result`
fn ok_example() {
    println!("\nBasic Ok()");
    let to_parse = "1";

    // `parse()` parses a string slice into another type
    // Howefver, `parse()` returns a `Result` which needs to be unwrapped
    let result = to_parse.parse::<i32>();
    println!("Our parsed attempt returned: {:?}", result);
}

/// Err(error) is an unsuccessful `Result`
fn err_example() {
    println!("\nBasic Err()");
    let to_parse = "    2    ";

    // `parse()` parses a string slice into another type
    // `parse()` returns a `Result` which needs to be unwrapped
    let result = to_parse.parse::<i32>();
    println!("Our parsed attempt returned: {:?}", result);
}

/// `unwrap()` will automatically retrieve the inner value
fn unwrap_example() {
    println!("\nBasic unwrap()");
    let to_parse = "3";

    // `parse()` parses a string slice into another type
    // `parse()` returns a `Result` which needs to be unwrapped

    let num_or_err = to_parse.parse::<i32>().unwrap(); // get the value out of the Ok()
    println!("Our parsed attempt returned: {:?}", num_or_err);

    // NOTE: `unwrap()` will panic if the value is an error!
}

/// `panic!()` automatically stops runtime if an error is encountered
fn panic_example() {
    println!("\nBasic panic!()");
    let to_parse = "5";

    // Notice how our variable is guaranteed to be i32 now
    let result: i32 = to_parse.parse::<i32>().unwrap();

    if result == 42 {
        println!("Our parsed attempt returned: {:?}", result);
    } else {
        panic!(
            "Oh no! I parsed {:?} but it isn't 42. That won't do",
            to_parse
        );
    }
}

/// `expect()` will panic if an error is encountered
fn expect_example() {
    println!("\nBasic expect");
    let to_parse = "    6    ";

    // Notice how our variable is guaranteed to be i32 now
    let number: i32 = to_parse.parse::<i32>().expect("Failed to parse number");

    println!("Our parsed attempt returned: {:?}", number);
}

/// `match` can be used, but it's very verbose
fn match_example() {
    println!("\nBasic match");
    let to_parse = "7";

    // `parse()` returns a `Result` which needs to be unwrapped
    let result = to_parse.parse::<i32>();

    // `match` arms MUST be exhaustive. Luckily `Result` only has 2 cases
    match result {
        Ok(some_num) => println!("Our parsed attempt returned: {:?}", some_num),
        Err(e) => panic!("Error: {}", e),
    };
}

// `if let` can be used to handle a single `match` case
fn if_let_example() {
    println!("\nBasic if-let");
    let to_parse = "8";

    // `parse()` returns a `Result` which needs to be unwrapped
    let result = to_parse.parse::<i32>();

    // This reads "if our result is of type `Ok()`"
    if let Ok(some_num) = result {
        println!("Our parsed attempt returned: {:?}", some_num);
    }
}
