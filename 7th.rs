//Implement a function that returns the kth smallest element in a given array.

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

fn main() {
    let arr = [4, 2, 7, 1, 9, 5];
    let k = 3;

    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
