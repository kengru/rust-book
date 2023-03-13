fn main() {
    let celsius = fahrenheit_to_celsius(32);
    println!();
}

fn fahrenheit_to_celsius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}
