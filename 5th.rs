//Given a sorted array of integers, implement a function that returns the median of the array.


use std::io;

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter valid integers"))
        .collect();

    let median_val = median(&arr);

    println!("Median of the array: {}", median_val);
}
