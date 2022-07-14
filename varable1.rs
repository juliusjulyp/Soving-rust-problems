// x and y were declared but not initialised any value at first
// I solved the problem by assigning x and y a value which must be equal so as to get a suuccess as our output
fn main() {
    let x: i32 = 5; 
    let y: i32 = 1+4;
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

    
    //third quiz(scope)
    //y was not able to be accesed by the statement since it was inside it, 
    fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}
    
    
    //shadowing
    // made sure that when calling assert_eq! values should be the same
    fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x =  42;
    println!("{}", x); // Prints "42".
}

    
    //shadowing
    
    
   fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
} 
    
    
    
    
    
    
    
    
    
    
    
    
