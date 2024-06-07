use std::io:

fn main() {
    let x: u8 = 12; // 0 - 255
    let y: i8 = 10; // -128 - 127

    let z = x + y; // Won't work because types are not the same
    println!("{}", z);
    
    let x2: f64 = 255.0; // 0 - 255
    let y2: f64 = 10.0; // -128 - 127

    let z2 = x2 / y2;
    println!("{}", z2);
    
    let z3 = x2 * y2;
    println!("{}", z3);
    
    let z4 = x2 % y2; // Gives remainder after division of the values
    println!("{}", z4);

    // Type Casting and Conversion
    let x3 = 255.0f32; 
    let y3 = 10.0f32; 
    
    let x4 = 127_i8; 
    let y4 = 10_i8; 

    let z5 = x3 % y3; 
    println!("{}", z5);
    
    let z6 = x4 % y4; 
    println!("{}", z6);

    let x5 = 127_000i64; 
    // let x5 = 127_000 as i64; 
    let y5 = 10_i64; 

    let z7 = x5 / y5; 
    println!("{}", z7);

    let x6 = 127_000 as i64;
    let y6 = 10_i32;

    let z8 = x / (y as i64); // Don't need '()' but can if you prefer
    println!("{}", z8);

    // Converting String to Integer
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2);
}
