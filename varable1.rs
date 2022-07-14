// x and y were declared but not initialised any value at first
// I solved the problem by assigning x and y a value which must be equal so as to get a suuccess as our output
fn main() {
    let x = 5; 
    let y = 1+4;

    assert_eq!(x, y);
// assert_eq! is used to check equality of two expressions
    println!("Success!");

    
    //second quiz
    //mut is used for reassignment
   fn main() {
    let mut x =  1;
       //x+2 adds one to the value of x
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}
