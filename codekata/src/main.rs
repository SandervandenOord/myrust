// fn double_char(s: &str) -> String {
//     let mut repeat_string = String::with_capacity(s.len() * 2);
//     s.chars().flat_map()
//     for c in s.chars() {
//         repeat_string.push(c);
//         repeat_string.push(c);
//     }
//     return repeat_string;
// }

fn main() {
    let input = "Hi world!";
    let output: String = input.chars().map(|c| [c, c]).flatten().collect();
    println!("{output}")
    // let output = double_char(input);
    // println!("{output}");
}
