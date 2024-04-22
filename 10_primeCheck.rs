use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // Read input from the user
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: u32 = input.trim().parse().expect("Not a valid number!");

    // Call the is_prime function and print the result
    if is_prime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}
