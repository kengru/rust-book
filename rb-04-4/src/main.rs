fn main() {
    let wordest = String::from("Option of the highest value");
    let i = first_word(&wordest);
    println!("{i}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
