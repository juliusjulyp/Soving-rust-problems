fn main() {
    print();
}

// replace i32 with a binary operator
fn print() -> () {
    println!("succes");
}


//assigning false to a value

fn main() {
    // FILL in the blank
    let b = false;

    let v = match b {
        true => 1,
        // Diverging functions can also be used in match expression
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
