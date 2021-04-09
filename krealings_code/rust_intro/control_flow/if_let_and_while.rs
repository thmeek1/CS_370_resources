fn main() {
    // An `Option` is an enum that represents either a value 'Some' or nothing 'None'
    // from std::option::Option
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }

    //let mut optional = None; 
    let mut optional = Some(5);// optional can be changed, MUTated

    // `if let` can be used to match a single case
    if let Some(num) = optional {
        println!("Got {}", num);
    } else {
        println!("nothing");
    }

    while let Some(num) = optional {
        if num == 10 {
            optional = None;
            break;
        }
        optional = Some(num + 1);
        println!("Optional is now {:?}", optional);
    }
    println!("DONE: Optional is now {:?}", optional);
}
