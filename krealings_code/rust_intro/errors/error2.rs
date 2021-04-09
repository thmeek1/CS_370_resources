fn main() {
    // Many functions return an `Result` enum, which is either
    //      Ok(some value)
    //      Err(some error)
    // There are plenty of ways to "unwrap" and obtain the inner data

    let num = unwrap_example().unwrap();

    println!("Num is {:?}", num);
}

/// `unwrap()` will automatically retrieve the inner value
fn unwrap_example() -> Result<i32, std::num::ParseIntError> {
    println!("\nBasic unwrap()");

    let to_parse = "5";
    //When applied to values of the Result<T, E> type, it propagates errors. If the value is Err(e), then it will
    //return Err(From::from(e)) from the enclosing function or closure. If applied to Ok(x), then it will unwrap the
    //value to evaluate to x.
    let num = to_parse.parse::<i32>()?;
    Ok(num)
















    //Ok(to_parse.parse::<i32>()?)
}
