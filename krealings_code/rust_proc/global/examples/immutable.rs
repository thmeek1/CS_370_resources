
static VALUE: i32 = 999;


fn main() {

    println!("I can see a beautiful value with {}", VALUE);
    let value = 907; // case makes 'value' a different variable name than 'VALUE'
    println!("I can see a beautiful value with {}", value);

    foo()
}

fn foo() {
    println!("FOO can see a beautiful value with {}", VALUE);
    let value = 807;
    println!("FOO can see a beautiful value with {}", value);

}
