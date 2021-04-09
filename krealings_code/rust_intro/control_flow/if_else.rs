fn main() {
    /* If statements are what you'd expect

     * if boolean expression is true {
     *    do this 
     *  } else { 
     *    do that
     *  }
     */
    // Parenthesis are optional (and rust will complain if they are not really needed.)
    // Curly braces are not optional.
    if 1 == 2 {
        println!("Iron Man is better than Captain America")
    } else if {
        
        let x = 2;
        let y = 3;
        x == y   // The last value of the expression is the evaluation, because no ';'
    } {
        println!("Captain America is better than Iron Man");
    } else {
        println!("Spiderman is better than both");
    }

    // You can declare variables using the resut of an if-else
    let x = if 1 == 2 { "first case" } else { "second case" };
    println!("X is {}", x);
}
