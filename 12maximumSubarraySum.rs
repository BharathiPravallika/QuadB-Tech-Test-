use std::io;

fn main() {
    // Read array from the user
    println!("Enter elements of the array (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.trim().parse().expect("Not an integer!"))
        .collect();

    // Calculate and print the maximum subarray sum
    let max_sum = arr.iter().fold((arr[0], arr[0]), |(max_sum, current_sum), &num| {
        let current_sum = num.max(current_sum + num);
        (max_sum.max(current_sum), current_sum)
    }).0;
    println!("Maximum subarray sum: {}", max_sum);
}
