fn main() {
    double_chars_challenge();
    dna_to_rna_challenge();
}

fn double_chars_challenge() {
    let input = "Hi world!";
    let output: String = input.chars().map(|c| [c, c]).flatten().collect();
    println!("{output}");
    let output = double_char(input);
    println!("{output}");
}

fn double_char(s: &str) -> String {
    let mut repeat_string = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        repeat_string.push(c);
        repeat_string.push(c);
    }
    return repeat_string;
}

fn dna_to_rna_challenge() {
    let rna = dna_to_rna("TABT");
    println!("{rna}");

    let rna = dna_to_rna_alternative("TABT");
    println!("{rna}");
}

fn dna_to_rna(dna: &str) -> String {
    let mut rna = String::new();
    for char_ in dna.chars() {
        if char_ == 'T' {
            rna.push('U')
        } else {
            rna.push(char_)
        }
    }
    rna
}

fn dna_to_rna_alternative(dna: &str) -> String {
    dna.replace("T", "U")
}
