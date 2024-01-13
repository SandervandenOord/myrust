pub fn double_chars_challenge() {
    let input = "Hi world!";

    let output = double_char(input);
    println!("{output}");

    let output: String = double_char_alternative(input);
    println!("{output}");
}

fn double_char(s: &str) -> String {
    let mut repeat_string = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        repeat_string.push(c);
        repeat_string.push(c);
    }
    repeat_string
}

fn double_char_alternative(s: &str) -> String {
    s.chars().map(|c| [c, c]).flatten().collect()
}
