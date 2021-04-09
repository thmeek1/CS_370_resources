// A struct is like an object without the methods
// An unordered collection or linking of data

#[derive(Debug)] // for printing
struct Cube {
    height: u32, // Fields are represented like function parameters
    width: u32,  // `data name: data type,`
    depth: u32,  // Data types don't have to be the same, just for this example
}

fn main() {
    // Structs can be created via `struct literal` declarations
    let cube = Cube {
        height: 10,
        width: 4,
        depth: 3,
    };

    // Fields are obtained via `.`
    let volume = cube.height * cube.width * cube.depth;

    println!("Our cube is: {:#?}", cube); // Note the # for formatted display
    println!("\nWith a volume of {}", volume);
}
