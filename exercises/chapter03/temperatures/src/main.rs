fn main() {
    let celsius = fahrenheit_to_celsius(57);
    let fahrenheit = celsius_to_fahrenheit(11);
    println!("57°F is {celsius}°C.");
    println!("10°C is {fahrenheit}°F.");
}

fn fahrenheit_to_celsius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn celsius_to_fahrenheit(c: i32) -> i32 {
    (c * 9 / 5) + 32
}
