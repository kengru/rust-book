//! Convert strings to pig latin. The first consonant of each
//! word is moved to the end of the word and “ay” is added, so
//! “first” becomes “irst-fay.” Words that start with a vowel
//! have “hay” added to the end instead (“apple” becomes “apple-hay”).
//! Keep in mind the details about UTF-8 encoding!

fn main() {
    let model = String::from("The apple fell down i from the biggest tree");
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut new_phrases = vec![];

    for w in model.split_whitespace() {
        let first = &w[..1];
        let pig = if vowels.iter().any(|&v| w.starts_with(v)) {
            format!("{}{}", w, "-hay")
        } else {
            format!("{}{}{}{}", &w[1..], "-", first, "ay")
        };
        new_phrases.push(pig);
    }

    let pig_latin = new_phrases.join(" ");
    println!("{}", pig_latin);
}
