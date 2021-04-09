// An enum represents a data type and variants

#[derive(Debug, Clone, Copy)]
enum Grade {
    A = 4,
    B = 3,
    C = 2,
    D = 1,
    F = 0,
}

fn main() {
    let my_grade = Grade::B;
    println!("my grade is {:?} worth {} quality points", my_grade, my_grade as u32);

}
