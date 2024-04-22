use std::io;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    // Read input from the user
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Call the reverse_string function and print the result
    let reversed = reverse_string(input.trim());
    println!("Reversed string: {}", reversed);
}
