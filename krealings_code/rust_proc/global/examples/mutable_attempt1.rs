// This is not legal because IFF we have multple threads danger comes threads all share the SAME static area,
// DUN DUN DUN!!!
static mut VALUE: i32 = 999;


fn main() {

    VALUE += 1;
    println!("I can see a beautiful value with {}", VALUE);
    let local_value = 907;
    println!("I can see a beautiful value with {}", local_value);

    foo();
}

fn foo() {
    println!("FOO can see a beautiful value with {}", VALUE);
    let local_value = 807;
    println!("FOO can see a beautiful value with {}", local_value);

}
