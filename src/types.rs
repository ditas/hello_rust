/*
Primitive types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: bool
Char: char
Tuples: (...)
Arrays: ??? (fixed number of elements)
 */

// Rust is statically typed lang (have to know the type at compile time, but can get it from the value)
pub fn run() {
    let a = 1; // i32
    let b = 0.25; // f64
    let c: i64 = 123;

    // get the max val for type
    println!("max for i32 is {}", std::i32::MAX);

    let is_true = true;
    println!("{:?}", (a,b,c,is_true));

    let is_greater = 10 > 5;
    println!("{}", is_greater);

    // Chars
    let a1 = 'a';
    let face = '\u{1F600}'; // \u - for unicode, it's an emoji and it's a char
    println!("{}", face);
}