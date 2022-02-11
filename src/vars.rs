// Variables hold primitive data or refences to data 
// Variables are immutable by default
// Rust is block-scoped langauge - meaning if a variable is set in a function, its only for the scope of that function context

pub fn run() {
    let name = "Courtney";
    let mut age = 31; // makes the variable mutable
    println!("My name is {} and i am {}", name, age);  // used to avoid waring for unread variable before state change 

    age = 33;
 
    println!("My name is {} and i am {}", name, age);

    // Difine Const
    const ID: i32 = 001; 

    println!("ID: {}", ID);

    // Assign Multiple Variables
    let (my_name, my_age) = ("Courtney", 33);
    
    println!("{} is {}", my_name, my_age)

}