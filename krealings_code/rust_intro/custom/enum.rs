// An enum represents a data type and variants

#[derive(Debug)] // attribute, basically gives you default implementation of Debug (formats a value)
enum OptionalNum {
    None,         // A value can be standalone
    SomeNum(i32), // Or have associated data
}

fn main() {
    let a_num = OptionalNum::SomeNum(5);
    println!("A num: {:?}", a_num);

    let no_num = OptionalNum::None;
    println!("No num: {:?}", no_num);
}
