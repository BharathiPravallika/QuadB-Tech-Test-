use std::io;

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        prefix.truncate(
            prefix
                .chars()
                .zip(s.chars())
                .take_while(|(a, b)| a == b)
                .count(),
        );
    }
    prefix
}

fn main() {
    // Read the vector of strings from the user
    println!("Enter a vector of strings (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let strs: Vec<String> = input
        .split_whitespace()
        .map(|x| x.trim().to_string())
        .collect();

    // Find and print the longest common prefix
    let result = longest_common_prefix(&strs);
    println!("The longest common prefix is: {}", result);
}
