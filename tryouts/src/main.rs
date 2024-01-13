fn main() {
    let mut x = "String literal".to_owned();
    let y = String::from("some text");
    x = x + "test";
    println!("{y}");
    println!("{x}");

    let s1 = "Hello";
    let s2 = " World";
    let result = s1.to_owned() + s2;
    println!("{result}");
}
