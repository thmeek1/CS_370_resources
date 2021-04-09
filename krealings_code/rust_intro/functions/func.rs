// Like other imperative languages, main is executed first
fn main() {
    // Basic function call
    do_stuff();

    // Passing parameters
    parameters(false, 10);

    // Using a return value
    let x = returned();
    println!("Got {} from function!", x);
}

// Function declarations are simple
fn do_stuff() {
    println!("I am doing stuff");
}

// Parameters are declared `name: datatype`, separated by commas
fn parameters(data: bool, other_data: i32) {
    println!("I was passed {} and {}", data, other_data);
}

fn returned() -> i32 {
    let val = 10;
    println!("I will return {}", val);

    // The final line in any function is treated as its return in the absence of the ending semicolon
    // You can also explicitly `return val;`
    val
}
