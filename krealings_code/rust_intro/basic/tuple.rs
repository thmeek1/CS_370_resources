fn main() {
    // Tuples are fixed-size containers that can hold multiple data types
    let mut tuple = (1, 2.0, "three", false);

    //tuple.push("cat");

    // You have to use "debug formatting" to print a tuple
    println!("\nTuple is {:?}\n", tuple);

    // Indexed via .
    let first_item = tuple.0;
    let second_item = tuple.1;
    println!(" Pull out items via indexing ");
    println!("\titem 1: {} ------ item 2: {}\n", first_item, second_item);

    tuple.0 = 7;
    println!("\nTuple is {:?}\n", tuple);

    // You can "destructure" a tuple
    let (first, second, third, fourth) = tuple;

    println!(" Destructure a tuple ");
    println!("\t{} --- {} --- {} --- {}\n", first, second, third, fourth);

}
