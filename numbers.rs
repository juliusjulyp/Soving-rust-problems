//for y to be equal to x both must be i32
fn main() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}


fn main() {
    let v: u16 = (38_u8 as i32).try_into().unwrap();

    println!("Success!");
}



fn main() {
    let x = 5;
// it should be i32 and not u32, since we not working with unsigned integer
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
