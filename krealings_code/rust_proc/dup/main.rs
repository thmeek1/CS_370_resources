use std::fs::File;
use std::io::{Error, Write};

fn main() {

    attempt("example1.txt").expect("Failed in `attempt`");

    correct("example2.txt").expect("Failed in `correct`");
}

/// Attempting to open the same file twice
fn attempt(filename: &str) -> Result<(), Error> {
    // Intuitively, you'd think you could open the same file twice
    let mut file = File::create(filename)?;
    file.write(b"Coming from FILE #1\n")?;

    //let mut duped = File::open(filename)?;
    let mut duped = File::create(filename)?;
    duped.write(b"Coming from DUPED #1\n")?;

    Ok(())
}


























/// Correct way; duping file handle
fn correct(filename: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    file.write(b"Coming from FILE #2\n")?;

    // The solution is to clone the file handle
    let mut duped = file.try_clone()?;
    duped.write(b"Coming from DUPED #2\n")?;
    duped.write(b"moo\n")?;
    duped.write(b"moo\n")?;



    file.write(b"MEOW\n")?;
    // files are closed when the go out of scope, normally I would not call this

    Ok(())
}
