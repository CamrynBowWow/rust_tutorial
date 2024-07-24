fn main() {
    println!("Hello, world!"); // Expression
    test();
    add_numbers(20, 30); // Expression

    let x = 20; // Statement
    let x2 = 2 < 3; // Expression because it evaluates to true or false

    // Expression
    let number = {
        let x = 3;
        x + 1
    }; // Don't put semicolon at end because it is returning a value from the block
    // If semicolon is put at the end inside the block it will return an error
    // Semicolon at end makes it a statement. No Semicolon makes it an expression

    println!("{}", number);

    let result = add_numbers_two(2, 3);
    println!("{}", result);
    
    let result2 = add_numbers_three(5, 3);
    println!("{}", result2);
}

// Use snake case
// test_one
fn test() {
    println!("Test has been called...");
}

// Function Parameters
fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

// Statements vs Expressions
// Expression is something that evaluates to something or returns a value 

// Returning From Function
fn add_numbers_two(x: i32, y: i32) -> i32 {
    x + y
    // return x + y; // Can also do this
}

// Can do that as its the expression being returned at the end
fn add_numbers_three(x: i32, y: i32) -> i32 {
    let result = x + y;
    result
}