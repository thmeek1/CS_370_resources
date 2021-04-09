fn main() {
    // Printing uses the a print macro
    println!("1) I will be displayed to stdout, Cool beans!");

    eprintln!("2) Stderr! here I come!!");

    // Automatically append a newline
    println!("\n3) Coffee is better than tea");

    // Formatting is done via {}
    println!("4) Some data is {}", 10);

    // Printing to standard err is the same
    eprintln!("5) This will go to stderr! Here's a number {}", 5);
}
