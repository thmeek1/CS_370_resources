fn main() {
    // Arrays are fixed-size containers that can hold a single data type
    let array = [1, 2, 3]; // type inference
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // type annotations
    let a: [u8; 100] = [0; 100]; // initialization

    // Indexed via []
    let first = array[0];



    // Array size MUST be known at compile time
    //for i in 0..10 {
        //let array: [i32; i];
    //}

    // So, they know their lengths
    let len = array.len();

    // You have to use "debug formatting" to print an array
    println!("array is {:?}", array);
    println!("arr is {:?}", arr);

    let [one, two, three] = array;
    println!("one: {}, two: {}, three: {}", one, two, three);
    println!("array is {:?}", array);


    // We cannot do this, because printing large arrays (by default) is not supported (yet)
    //println!("a is {:?}", a);
















    let mut count = 0;
    for i in 0..a.len() {
       print!("{} ",a[i]);
       if count >= 50 {
          count = 0;
          println!();
       } else {
         count += 1;
       }
    }
    println!();

}
