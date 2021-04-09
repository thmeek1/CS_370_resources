// A `trait` is a collection of methods for an unknown type
// Very similar to an interface (You are most probably familiar with Java Interfaces)

// define a new trait
trait Automobile {
    type Color; // Type Color is a placeholder (look in impl)
    
    // `Self` is the object type
    fn new() -> Self; 

    // This function will return a value
    fn color(&self) -> &Self::Color;

    // This will modify the object, self needs to be mutable to allow modification
    fn repaint(&mut self, new_color: Self::Color);
}

#[derive(Debug)]
struct Car<'a> {
    color: &'a str,
}

// We must manually implement ALL of the `automobile` methods
impl<'a> Automobile for Car<'a> {
    type Color = &'a str; // Here we are defining Color

    fn new() -> Car<'a> {
        Car { color: "black" }
    }

    fn color(&self) -> &Self::Color {
        &self.color
    }

    fn repaint(&mut self, new_color: &'a str) {
        self.color = new_color;
    }
}

#[derive(Debug)]
struct Truck<'b> {
    color: &'b str,
    wheels: u64,
}

// We must manually implement ALL of the `automobile` methods
impl<'b> Automobile for Truck<'b> {
    type Color = &'b str;

    fn new() -> Truck<'b> {
        Truck {
            color: "Grey",
            wheels: 6,
        }
   }

    fn color(&self) -> &Self::Color {
        &self.color
    }

    fn repaint(&mut self, new_color: &'b str) {
        self.color = new_color;
    }
}

fn main() {
    let mut truck = Truck::new();
    println!("Truck: {:?}", truck);

    truck.repaint("green");

    println!("Truck: {:?}", truck);

    let car = Car::new();
    let current = car.color();
    println!("let me see the color ==> {}", current);

    println!("Car: {:?}", car);
}
