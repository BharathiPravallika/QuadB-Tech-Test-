use std::io;

fn main() {
    // Read the sorted array from the user
    println!("Enter a sorted array of integers (space-separated):");
    let mut sorted_array = String::new();
    io::stdin().read_line(&mut sorted_array).expect("Failed to read line");

    // Read the target number from the user
    println!("Enter the number to search for:");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target: i32 = target.trim().parse().expect("Not an integer!");

    // Find the first occurrence of the target number in the array
    if let Some(index) = sorted_array
        .split_whitespace()
        .position(|x| x.parse::<i32>().unwrap() == target)
    {
        println!("The first occurrence of {} is at index {}.", target, index);
    } else {
        println!("{} does not exist in the array.", target);
    }
}
