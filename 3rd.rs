//Given a string of words, implement a function that returns the shortest word in the string.
fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn main() {
    let input_string = " will i get this internship ";
    let shortest = shortest_word(input_string);
    println!("The shortest word in the string is: {}", shortest);
}
