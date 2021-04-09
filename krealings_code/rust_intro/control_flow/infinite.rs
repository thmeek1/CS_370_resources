// Infinite loops
fn main() {

    let mut x = 0;

    loop {
        x += 1;

        if x % 2 == 0 {
            println!("{} is even", x);

            // Skip the rest of this loop iteration AKA
            // continue with the NEXT iteration
            continue;
        }
        println!("{} is odd", x);

        // Break out of the loop
        if x == 11 {
            break;
        }
    }

    println!("\n\nAbout to set y and x is: {}", x);
    // Loops can be used to declare variables
    let y = loop {
        x -= 1;

        if x == 0 {
            // `break` acts similarly to `return`
            break x;
        }
    };

    println!("y is now {}", y);
}
