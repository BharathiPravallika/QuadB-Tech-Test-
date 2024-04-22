use std::io;

fn main() {
    // Read and merge sorted arrays from the user
    let mut read_array = |message: &str| -> Vec<i32> {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.split_whitespace()
            .map(|x| x.trim().parse().expect("Not an integer!"))
            .collect::<Vec<i32>>()
    };

    let arr1 = read_array("Enter elements of the first sorted array (space-separated):");
    let arr2 = read_array("Enter elements of the second sorted array (space-separated):");

    let merged = arr1.iter().chain(&arr2).copied().collect::<Vec<_>>();
    println!("Merged array: {:?}", merged);
}
