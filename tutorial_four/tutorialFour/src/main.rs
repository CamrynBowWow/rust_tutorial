fn main() {
    println!("Hello, world!");

    let x: i32 = 2;
    // Different Integer types
    // i8
    // i16
    // i32
    // i64
    // i128

    // Unsigned Integer 'Can not use negatives'
    let x1: u32 = 972;
    // Different Unsigned Integer types
    // u8
    // u16
    // u32
    // u64
    // u128

    // Floating Points
    // Types // Default type is f64
    // f32
    // f64
    let floating_point: f32 = 10.9;

    // Booleans
    // true or false || 1 or 0
    let true_or_false: bool = true;

    // Char
    let letter: char = 'a';

    // Tuples
    let tup: (i32, bool, char) = (1, true, 's');
    let mut tup2: (i8, bool, char) = (1, true, 's');

    // Use index of element you want to reference
    println!("{}", tup.0);
    println!("{}", tup.2);

    tup2.0 = 2;
    println!("{}", tup2.0);
    
    tup2 = (3, false, 'a');
    println!("{}", tup2.0);
    println!("{}", tup2.1);
    println!("{}", tup2.2);

    // Arrays
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4]);
}
