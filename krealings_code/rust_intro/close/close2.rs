fn main() {
    let num = 99;

    fn equal_to_num(data: i32) -> bool {
        data == num 
    }

    let value = 99;

    assert!(equal_to_num(value));
}
