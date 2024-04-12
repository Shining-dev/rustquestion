//Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = String::new();
    'outer: for (i, c) in strings[0].chars().enumerate() {
        for string in &strings[1..] {
            if let Some(other_c) = string.chars().nth(i) {
                if c != other_c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }
    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    let common_prefix = longest_common_prefix(&strings);

    println!("Longest common prefix: {}", common_prefix);
}
