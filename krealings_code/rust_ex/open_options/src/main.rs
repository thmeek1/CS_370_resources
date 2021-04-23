use std::fs::OpenOptions;

fn main() {
    // OpenOptions provides several options for file creation.
    // The `open` method returns a `File`, so you can read lines using a
    // BufReader in the same way

    // Creates a new file, or opens if it already exists
    let _create_or_open = OpenOptions::new()
        .create(true)
        .write(true) // `write` or `append` must be used to create a file
        .open("create_or_open.txt")
        .expect("Failed to create OpenOptions");

    // Opens the file for read-only mode (does not create)
    let _readable = OpenOptions::new()
        .read(true)
        .open("readable.txt")
        .expect("Failed to create OpenOptions");

    // Opens the file for writing (does not create)
    let _writable = OpenOptions::new()
        .write(true)
        .open("writeable.txt")
        .expect("Failed to create OpenOptions");

    // Opens the file for writing, overwriting all contents if it exsts (does not create)
    let _truncated = OpenOptions::new()
        .write(true) // Needed to set the file's length to 0
        .truncate(true)
        .open("truncated.txt")
        .expect("Failed to create OpenOptions");

    // Opens the file for writing, appending all new contents to the end (does not create)
    let _appended = OpenOptions::new()
        .append(true)
        .open("appended.txt")
        .expect("Failed to create OpenOptions");
}
