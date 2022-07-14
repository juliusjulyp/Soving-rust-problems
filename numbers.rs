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



//what equals max of i8 and also u8

fn main() {
    //the largest value that can be represented by i8 will be 127i8
    assert_eq!(i8::MAX, 127i8); 
     //the largest value that can be represented by u8 will be 255u8
    assert_eq!(u8::MAX, 255u8); 

    println!("Success!");
}



// f64 represents the sign of self 
fn main() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    println!("Success!");
}


//floating point numbers

fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
}
fn main() {
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);
}


//computations

fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    //  subtraction of integers
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    
    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); 

    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // all bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}












