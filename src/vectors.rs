// Vectors  - Resizable Arrays
use std::mem;

pub fn run() {
    let numbers: Vec<i32> = vec![1,2,3,4,5]; // Defines an array of 5 elements 
    println!("{:?}", numbers);

    // Get Single value 
    println!("Single Value: {}", numbers[0]); // Zero Index


    // Mutable Arry 
    let mut m_numbers: Vec<i32> = vec![1, 2, 3];

    // Reassign Value
    m_numbers[1] = 20;
    println!("{:?}", m_numbers);

    // Get Array Len
    println!("Vector Len: {}", numbers.len());

    // Arrays are stack allocated  - Get memory 
    println!("Vector Occupies {} bytes", mem::size_of_val(&m_numbers));

    // Get Slice
    let slice: &[i32] = &m_numbers[0..2];

    println!("Slice: {:?}", slice)
}