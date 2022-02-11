// Two types of strings 
// Primitive str = Immutable fixed-len string somewhere in memory 
// String = Growable, heap-allocated data structure- Use when you need to modify or own string data 

pub fn run() {
    // Primitive String 
    let hello = "Hello";

    // Growable String
    let mut hello_string = String::from("Hello ");

    // Get len of string 
    let hello_len = hello.len();

    // Adding to string 
    hello_string.push('W'); // Only char literal 
    hello_string.push_str("orld!"); // Used to push strings

    // Get capacity in bytes- number of bytes the string can store 
    println!("Capacity: {}", hello_string.capacity());

    // Check is empty 
    println!("Is Empty: {}", hello_string.is_empty());

    // Check if it contains substring 
    println!("Does it Contains world?: {}", hello_string.contains("World"));

    // Replace 
    println!("Replaced: {}", hello_string.replace("World", "There!"));


    // Loop thorugh string by whitespace 
    for word in hello.split_whitespace(){
        println!("{}", word)
    }
    
    // Create string with set Capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s.capacity());

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());



    println!("{:?}", (hello, hello_string, hello_len, s))
}