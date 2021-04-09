// Rust has objects! You can attach methods to structs

#[derive(Debug)] // for printing
struct Cube {
    height: u32,
    width: u32,
    depth: u32,
}

// Here we implement associated functions and methods
impl Cube {
    // Technically, this is a static method, as it has no 'self' reference
    // AKA it has no reference to the object that contains it
    // by convention we would call this a constructor
    fn new(height: u32, width: u32, depth: u32) -> Cube {
        Cube {
            height: height, // You can declare the long way (field is assigned the value in the parameter)
            width,          // Or use shorthand since the params' names match the fields' names
            depth,
        }
    }

    // This function takes a self reference, so it is a method
    fn find_volume(&self) -> u32 {
        self.height * self.width * self.depth // accessing the fields
    }

    // To change the object, we need a mutable reference
    fn expand(&mut self) {
        self.height *= 2;
        self.width *= 2;
        self.depth *= 2;
    }
}

fn main() {
    // We can use our `new` function now
    let mut cube = Cube::new(3, 6, 7); // if I do not call expand, this does not need to be mutable
                                       // and the compiler will let me know!

    // Methods are called via `.`
    let volume = cube.find_volume();

    println!("Our cube is: {:#?}", cube); // Note the # for formatted display
    println!("\nWith a volume of {}", volume);

    cube.expand();
    let volume = cube.find_volume();

    println!("\n\nAfter expanding:");
    println!("Our cube is: {:#?}", cube); // Note the # for formatted display
    println!("\nWith a volume of {}", volume);
}
