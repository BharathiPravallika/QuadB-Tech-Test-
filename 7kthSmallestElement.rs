use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted.get(k - 1).cloned()
}

fn main() {
    // Read the array of integers from the user
    println!("Enter an array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.trim().parse().expect("Not an integer!"))
        .collect();

    // Read the value of k from the user
    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Not an integer!");

    // Find and print the kth smallest element
    if let Some(result) = kth_smallest(&arr, k) {
        println!("The {}th smallest element is: {}", k, result);
    } else {
        println!("Invalid input or k is out of range.");
    }
}
