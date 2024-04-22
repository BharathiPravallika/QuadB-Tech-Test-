fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted.get(k - 1).cloned()
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6];
    let k = 3;
    
    if let Some(result) = kth_smallest(&arr, k) {
        println!("The {}th smallest element is: {}", k, result);
    } else {
        println!("Invalid input or k is out of range.");
    }
}
