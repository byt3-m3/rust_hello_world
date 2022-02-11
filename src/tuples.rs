// Tuples are group of values of diffrent types 
// Max 12 Elements 

pub fn run() {

    // Must set type, &str= string literal 
    let person: (&str, &str, i8 ) = ("Courtney", "Orlando", 33);
    println!("{} is from {} and is {}", person.0, person.1, person.2)

}