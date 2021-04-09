// Constants *must* have type annotations
const CONSTANT: bool = false;

fn main() {
    // 'let' binds data to a variable
    let _var = "some data data";

    // Variables can have type annotations or you can let the compiler infer
    let _x = 2;
    let _y: i32 = 5;

    // If you don't know a data's type, make the compiler throw an error
    //let z: char = 100000000000000000000000000000000000000;

    // Variables are immutable by default- cannot be changed
    let q = 5;
    //q += 1;

    // Solve this via `mut`
    let mut p = 5;
    p += 1;

    // Typecasting works with `as`
    let a = 10.0;
    let _b = a as i32;
}
