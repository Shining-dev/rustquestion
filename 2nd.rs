//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    // Example usage:
    let sorted_arr = [1, 2, 2, 3, 3, 3, 4, 5, 5];
    let target = 5;
    match first_occurrence(&sorted_arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
