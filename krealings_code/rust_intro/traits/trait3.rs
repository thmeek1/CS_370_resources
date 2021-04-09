// A `trait` is a collection of methods for an unknown type
// Very similar to an interface (You are most probably familiar with Java Interfaces)

// define a new trait
trait Automobile {
    type Color;
    
    // `Self` is the object type
    fn new() -> Self; 

    // This function will return a value, but I just want to borrow it, not steal it
    fn color(&self) -> &Self::Color;

    // This will modify the object, self needs to be mutable to allow modification
    fn repaint(&mut self, new_color: Self::Color);
}

#[derive(Debug)]
struct Car {
    color: String,
}

// We must manually implement ALL of the `automobile` methods
impl Automobile for Car {
    type Color = String;

    fn new() -> Car {
        Car { color: "black".to_string() }
    }

    fn color(&self) -> &Self::Color {
        &self.color
    }

    fn repaint(&mut self, new_color: String) {
        self.color = new_color;
    }
}

#[derive(Debug)]
struct Truck {
    color: String,
    wheels: u64,
}

// We must manually implement ALL of the `automobile` methods

impl Automobile for Truck {
    type Color = String;

    fn new() -> Truck {
        Truck {
            color: "Grey".to_string(),
            wheels: 6,
        }
   }

    fn color(&self) -> &Self::Color {
        &self.color
    }

    fn repaint(&mut self, new_color: String) {
        self.color = new_color;
    }
}

fn main() {
    let mut truck = Truck::new();
    println!("Truck: {:?}", truck);

    truck.repaint("green".to_string());

    println!("Truck: {:?}", truck);

    let mut car = Car::new();
    let current = car.color();
    println!("The current color is {}", current);
    

    println!("Car: {:?}, I do not like it I will repaint", car);
    car.repaint("Maroon".to_string());
    println!("Car: {:?}", car);

}
