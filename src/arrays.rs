// Arrays  - Fixed list of the same type of elements 
use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5]; // Defines an array of 5 elements 
    println!("{:?}", numbers);

    // Get Single value 
    println!("Single Value: {}", numbers[0]); // Zero Index


    // Mutable Arry 
    let mut m_numbers: [i32; 3] = [1, 2, 3];

    // Reassign Value
    m_numbers[1] = 20;
    println!("{:?}", m_numbers);

    // Get Array Len
    println!("Array Len: {}", numbers.len());

    // Arrays are stack allocated  - Get memory 
    println!("Array Occupies {} bytes", mem::size_of_val(&m_numbers));

    // Get Slice
    let slice: &[i32] = &m_numbers[0..2];

    println!("Slice: {:?}", slice)
}