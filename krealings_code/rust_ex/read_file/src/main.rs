/// NOTE:
/// You MUST run this program from the root directory.
/// The file paths are relative to the root, so if you are in `/src/`, you
/// will have to modify the file paths accordingly
/// https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

fn main() {
    one_string();

    line_by_line();
}

/// Read an entire file to a string
fn one_string() {
    // Read the contents, panicking if it fails
    let contents = read_to_string("demo.txt").expect("Failed to read file");
    println!("------------------------------------------------\n");
    println!("Contents of file:\n{}", contents);
    println!("------------------------------------------------\n");
}

/// Read a file line-by-line
fn line_by_line() {
    // Open the file, panicking if not possible
    let file = File::open("demo.txt").expect("Failed to open file");

    // Create a new buffered reader for the file
    let reader = BufReader::new(&file);

    // Iterate over the lines of the reader
    println!("Contents of file:");
    for line in reader.lines() {
        // Note that each line is a `Result`
        println!("LINE: {}", line.unwrap());
    }
}
