pub fn run(){
    // print to console
    println!("Hello from the print RS file.");

    // Basic Formating
    println!("{} is from {}", "Courtney", "Orlando");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Courtney", "Orlando", "Code");

    // Named Arguments 
    println!("{name} likes to play {activity}",
     name = "Courtney",
      activity = "Coding"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello") );

    // Basic Math
    println!("10 + 10 = {}", 10 + 10)
}  
