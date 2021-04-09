// A `trait` is a collection of methods for an unknown type
// Very similar to an interface (You are most probably familiar with Java Interfaces)

// define a new trait
trait Automobile {
    // `Self` is the object type
    fn new() -> Self; 

    // This function will return a value
    fn color(&self) -> &'static str;

    // This will modify the object, self needs to be mutable to allow modification
    fn repaint(&mut self, new_color: &'static str);
}

#[derive(Debug)]
struct Car {
    color: &'static str,
}

// We must manually implement ALL of the `automobile` methods
impl Automobile for Car {
    fn new() -> Car {
        Car { color: "black" }
    }

    fn color(&self) -> &'static str {
        self.color
    }

    fn repaint(&mut self, new_color: &'static str) {
        self.color = new_color;
    }
}

#[derive(Debug)]
struct Truck {
    color: &'static str,
    wheels: u64,
}

// We must manually implement ALL of the `automobile` methods
impl Automobile for Truck {
    fn new() -> Truck {
        Truck {
            color: "Grey",
            wheels: 6,
        }
    }

    fn color(&self) -> &'static str {
        self.color
    }

    fn repaint(&mut self, new_color: &'static str) {
        self.color = new_color;
    }
}

fn main() {
    let mut truck = Truck::new();
    truck.repaint("green");

    println!("Truck: {:?}", truck);

    let car = Car::new();

    println!("Car: {:?}", car);
}
