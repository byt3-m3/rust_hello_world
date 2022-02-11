pub fn run() {
    greeting("Hello", "John");
    // Bind function values to variables
    let get_sum = add(5, 6);
    println!("Sum {}", get_sum);

    // Closure 
    let n3: i32 = 10;
    let add_numers = |n1: i32, n2: i32 | n1 + n2 + n3;

    println!("Closure Sum {}", add_numers(1, 2))
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // Not using a semi-colon indicates what we want to return 
}