//Implement a function that checks whether a given string is a palindrome or not.
use std::io;

fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    println!("Is '{}' a palindrome? {}", input, is_palindrome(input));
}
