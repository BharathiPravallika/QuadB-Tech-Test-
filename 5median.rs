use std::io;

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    // Read the array from the user
    println!("Enter an array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    // Calculate and print the median of the array
    let result = median(&arr);
    println!("The median of the array is: {}", result);
}
