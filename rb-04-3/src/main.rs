fn main() {
    let hello = return_a_string();
    let hello2 = return_a_literal();
    println!("{hello} {hello2}");

    let ken = vec![String::from("Kendry")];
    let new_name = stringify_name_with_title(&ken);
    println!("{new_name}");
}

fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}

fn return_a_literal() -> &'static str {
    "Hello world"
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
