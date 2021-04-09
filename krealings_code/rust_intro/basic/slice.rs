fn main() {
    // Like arrays, slices contain a single data type, but can be variably-sized
    let sample = [1, 2, 3, 4, 5, 6, 7, 8, 9 , 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
    println!("let sample = [1; 25]");
    println!("output {:?}", sample);
    println!();

    let slice = &sample[5..10]; // Contains a section of `sample`
    println!("let slice = &sample[5..10]");
    println!("output {:?}", slice);
    println!();
    
    let slice = &sample[..20];
    println!("let slice = &sample[..20]");
    println!("output {:?}", slice);
    println!();

    let slice = &sample[3..];
    println!("let slice = &sample[3..]");
    println!("output {:?}", slice);
    println!();


    let whole = &sample[..]; // Contains all of `sample` AKA a reference to `sample`
    println!("let whole = &sample");
    println!("output {:?}", whole);
    println!();

    // Slices know their lengths, too
    let len = slice.len();

    // You have to use "debug formatting" to print a slice
    println!("Slice has {} elements\nOutput: {:?}", len, slice);

}
