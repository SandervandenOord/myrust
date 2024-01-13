pub fn dna_to_rna_challenge() {
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
