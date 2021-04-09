fn main() {
    // When you declare a variable, that data is "owned" by that variable
    // Data will only have ONE owner in Rust

    let x = vec![1, 2, 3];

    // Data lasts until its owner goes out of scope
    {
        let _b = 10;
    }
    //println!("Can't print {} because it doesn't exist anymore!", b);

    // Here, the value of x is being "moved" into the variable y
    // y now "owns" the data
    let y = x;

    // This will throw an error, as x no longer owns the data
    //let z = x;

    // You can "borrow" data via references (`&`)
    let reference = &y;
    let other_ref = &y;

    // Data is automatically dereferenced, but you can explicitly use
    // the deref operator (and sometimes have to!)
    println!("Reference is: {:?}", reference);
    println!("Other ref is: {:?}", *other_ref);

    // References are immutable
    //*reference = String::new();

    // However, you can get a mutable reference from a mutable variable
    let mut mutable_data = Vec::new();
    let mut_ref = &mut mutable_data;

    mut_ref.push(1000);

    // Anytime data is "moved" it is assigned a new owner
    // so `x` will not exist after this function call
    let x = vec![true, true, false];
    take_ownership(x);
    //println!("\nx: {:?} doesn't exist anymore!", x);

    // the solution is to pass a reference to the data to function
    let mut y = vec![true, false, false];
    reference_data(&mut y);
    println!("\ny: {:?} still exists!", y);

    let z = vec![true, true, false];
    just_borrow(&z);
    println!("\nz: {:?} still exists but is not changed!", z);
}

// This function takes ownership of its parameter
fn take_ownership(data: Vec<bool>) {
    println!("\nHAHA! I own x: {:?} now!\n", data);

    // Once this function ends, the data "goes out of scope" and has no owner
    // therefore it is dropped- it no longer exists
}

// References can be mutable or immutable, like always
fn reference_data(data: &mut Vec<bool>) {
    println!("I only have a reference to {:?}", data);
    data[0] = false;
    println!("But I can change it to {:?}", data);
}

// References can be mutable or immutable, like always
fn just_borrow(data: &Vec<bool>) {
    println!("I only have a reference to {:?}", data);
}
