pub fn string_to_array_challenge() {
    let input = "Hello World";
    let output = string_to_array(input);
    println!("{:?}", output);
    let output = string_to_array_alternative(input);
    print!("{:?}\n", output)
}

fn string_to_array(s: &str) -> Vec<String> {
    s.split(" ").map(|x| x.to_string()).collect()
}

fn string_to_array_alternative(s: &str) -> Vec<String> {
    s.split_whitespace().map(str::to_string).collect()
}
