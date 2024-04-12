//Implement a function that checks whether a given number is prime or not.

use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false; // If n is divisible by any number between 2 and sqrt(n), it's not prime
        }
    }
    true // If no divisor is found, n is prime
}

fn main() {
    println!("Enter a number to check if it's prime:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse().expect("Please enter a valid number");

    if is_prime(number) {
        println!("{} is a prime number", number);
    } else {
        println!("{} is not a prime number", number);
    }
}
