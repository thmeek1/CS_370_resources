fn main() {
    let one = "Go";
    let two = "Cats!";

    let longer = longer_string(one, two);

    println!("The longer string is: {}", longer);
}

// Here we are ensuring that our returned reference will live at least as long
// as our parameters. 'a reads: "the lifetime of a"
fn longer_string<'a>(one: &'a str, two: &'a str) -> &'a str {
    if one.len() > two.len() {
        one
    } else {
        two
    }
}

// A that owns its data
struct OwnedString {
    data: String,
}

// A struct that holds a reference to data must have a lifetime associated
// with that data. The most common is `'static` which says that the reference
// could live for the life of the program
struct RefString {
    data: &'static str,
}
