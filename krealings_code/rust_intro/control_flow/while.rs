fn main() {
    let mut condition = 10;

    // While loops work like you'd expect
    while condition != 0 {
        condition -= 1;
        println!("value is {}", condition);
    }

    println!("Done, value is {}", condition);
}
