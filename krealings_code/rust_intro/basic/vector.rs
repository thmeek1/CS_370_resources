fn main() {
    // Vectors are arrays that can be dynamically sized:
    let _typed: Vec<char>;

    // Empty vector does not work without type annotations....??
    let _empty_untyped: Vec<i32> = Vec::new();
    //let _empty_untyped:  = Vec::new();






    // You can also use a macro for a quick declaration
    let shorthand = vec![1, 2, 3];

    // Indexed via []
    let _first = shorthand[1];

    // Push, pop, indexing, etc. is all relatively normal
    let mut vector = vec![1, 2, 3, 4];
    vector.pop();
    println!("Vector is {:?}", vector);
    vector.push(1);
    let _first = vector[0];

    // You have to use "debug formatting" to print a vector
    println!("Vector is {:?}", vector);
}
