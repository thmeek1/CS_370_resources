fn main() {
    // For loops are a like python
    for i in 0..10 {
        println!("In for loop, i = {}", i);
    }

    // You can also loop over lists, or anything iterable
    let items = vec!['W', 'C', 'U'];

    for item in items {
        print!("{}", item);
    }
    println!();
}
