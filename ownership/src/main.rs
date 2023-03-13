fn main() {
    let first = String::from("Ferris");
    let first_cloned = first.clone();
    let full = add_suffix(first_cloned);
    println!("{full}, originally {first}"); // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
