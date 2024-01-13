fn main() {
    let mut x = "String literal";
    let mut y = String::from("some text");
    x = x + "test";
    println!("{y}");
    println!("{x}");

    let s1 = "Hello";
    let s2 = " World";
    let result = s1.to_string() + s2;
}
