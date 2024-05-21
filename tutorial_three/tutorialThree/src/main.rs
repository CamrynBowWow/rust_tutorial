fn main(){
    // let x = 4;
    let mut x = 4; // 'mut' Allows the value to be changed
    println!("x is: {}", x);

    x = 5;
    println!("x is: {}", x);

    // Cannot assign twice to immutable variable
    // x = 5;
    // println!("x is: {}", x);

    // Can do
    let x = 4;
    println!("x is: {}", x);

    let x = 5;
    println!("x is: {}", x);
    
    let x = x + 1;
    println!("x is: {}", x);

    // Name Shadowing
    let x2 = 4;
    println!("x2 is: {}", x2);

    {
        let x2 = x2 - 2;
        println!("x2 is: {}", x2);
    }
    
    let x2 = x2 + 1;
    println!("x2 is: {}", x2);

    // Can do
    let x3 = 4;
    println!("x3 is : {}", x3);

    let x3 = "hello";
    println!("x is: {}", x3);

    // Constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}
