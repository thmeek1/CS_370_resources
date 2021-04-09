// Larger vector example

fn main() {

    // But wait, in the last example I said empty vectors MUST have type annotations?? WTH?
    let mut vector = Vec::new();
    
    for i in 0..100 {
        vector.push(i);
        let _first = vector[0];
    }

    // You have to use "debug formatting" to print a vector
    println!("Vector is {:?}", vector);
}
