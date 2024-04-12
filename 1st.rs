//Implement a function that checks whether a given string is a palindrome or not.


fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    // Test the is_palindrome function
    let test_cases = vec!["hash", "hello", "logic", "Palindrome", "A man "];
    for &test_case in &test_cases {
        println!("Is '{}' a palindrome? {}", test_case, is_palindrome(test_case));
    }
}
