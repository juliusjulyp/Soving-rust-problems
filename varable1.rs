// x and y were declared but not initialised any value at first
// I solved the problem by assigning x and y a value which must be equal so as to get a suuccess as our output
fn main() {
    let x = 5; 
    let y = 1+4;

    assert_eq!(x, y);
// assert_eq! is used to check equality of two expressions
    println!("Success!");
