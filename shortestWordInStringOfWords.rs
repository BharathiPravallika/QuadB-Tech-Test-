use std::io;

fn shortest_word(words: &str) -> &str {
    words.split_whitespace().min_by_key(|&word| word.len()).unwrap_or("")
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    println!("The shortest word in the sentence is: '{}'", shortest_word(input));
}
