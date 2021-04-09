fn main() {
    let variable;
    {
       let num = 10;

       variable = &num;
    }
    println!("The value of variable is {}", variable);
}
