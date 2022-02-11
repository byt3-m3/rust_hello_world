/* 
Primitive Types
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Number of bits they take in memory) - unsigned means no negative values
Floats: f32, f64
Boolean: (bool)
Characters: (char)
Tuples
Array - Fixed length, vectors are dynamic length

Rust is statitcally typed, but its not required to set the type during variable definition, the complier can infer the type based on the value 

*/


pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Explicit Type
    let z: i64 = 123456789123456789;

    // Find max size
    println!("Maix i32: {}", std::i32::MAX);
    println!("Maix i64: {}", std::i64::MAX);
    
    // Boolean 
    let is_actve = true;
    let is_not_actve: bool = false;


    // Get boolean from experession 
    let is_greater =  10 > 5;

    // Char - Chars use single quotes oppsoed to double quotes, has to be 1 character 
    let a1 = 'a';
    let face = '\u{1F600}'; // Emoji smilling face
    println!("{:?}", (x, y, z, is_actve, is_not_actve, is_greater, a1, face))

}