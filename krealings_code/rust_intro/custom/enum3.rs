// An enum represents a data type and variants

#[derive(Debug, Clone, Copy)]
enum Grade {
    F,
    D,
    C,
    B,
    A,
}

fn main() {
    let my_grade = Grade::B;
    println!("my grade is {:?} worth {} quality points", my_grade, my_grade as u32);

}
