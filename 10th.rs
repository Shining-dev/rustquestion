//Check if a number is prime in Rust
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    if n <= 3 {
        return true; // 2 and 3 are prime
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false; // Numbers divisible by 2 or 3 are not prime
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false; // Numbers divisible by i or i+2 are not prime
        }
        i += 6; // Increment by 6 to check only prime factors
    }
    true
}

fn main() {
    let number = 29;
    if is_prime(number) {
        println!("{} is a prime number", number);
    } else {
        println!("{} is not a prime number", number);
    }
}
