use std::io;

fn reverse_array<T: Clone>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
}

fn main() {
    println!("Enter the elements of the array, separated by spaces:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let mut arr = numbers.clone();
    println!("Original array: {:?}", arr);

    reverse_array(&mut arr);
    println!("Reversed array: {:?}", arr);
}