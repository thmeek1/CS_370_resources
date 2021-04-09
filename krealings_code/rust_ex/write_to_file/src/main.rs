use std::fs::{write, File};
use std::io::{Error, Write, Read};

/// Note that here we're using Propagation.
/// Propagation is when you tell a line of code to send its error to the
/// calling function, and is denoted by the `?` syntax in place of an
/// `unwrap()` or `expect()`. This makes error-prone functions, such as
/// file I/O, less verbose, as many points of failure must be addressed.
fn main() {
    let filename = "write_examples.txt";

    use_write(filename).expect("Failed to use write");

    create_file(filename).expect("Failed to create and write");

    open_file(filename).expect("Failed to open and write");
}

/// Using `write(), from std::fs`
fn use_write(filename: &str) -> Result<(), Error> {
    // `write()` writes a byte array, which can be obtained by the
    // `.as_bytes()` method on a `&str`
    write(filename, "Will this be overwritten".as_bytes())?;

    // `write()` can only write a byte array that is 32 bytes or shorter
    //write(filename, b"This message is too long to write")?;

    // You can also prefix the string with `b` to declare it as a byte array
    write(filename, b"Another message")?;

    // Since we're propagating the errors in this function, we must signal that
    // we have returned without errors
    Ok(())
}

/// Using `File::create()` and `write_all()`
fn create_file(filename: &str) -> Result<(), Error> {
    // `create()` will create the file if it does not already exist, else it
    // will open the existing file for writing only
    let mut created = File::create(filename)?;

    // `write_all()` writes a byte slice, which can be longer than 32
    created.write_all("This is super secret information\n".as_bytes())?;

    // `write()` works nearly identically to `write_all()`
    // The difference is that `write()` returns the number of bytes written
    created.write(b"Don't show anyone!")?;

    Ok(())
}

/// Using `File::open()`, and `read_to_string()`
fn open_file(filename: &str) -> Result<(), Error> {
    // `open()` will open an existing file for reading only, or throw an error if the file
    // does not exist
    let mut opened = File::open(filename)?;
    let mut buffer = String::new();

    // Again, you can prefix a `&str` with `b` to declare it as a byte slice
    //opened.write_all(b"Wait, it's all bytes?")?;
    opened.read_to_string(&mut buffer)?;
    println!("Here is what I got:\n {}", buffer);

    Ok(())
}
