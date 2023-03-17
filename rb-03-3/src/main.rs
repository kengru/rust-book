fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The returned function value: {x}");
    let y = plus_one(x);
    println!("The returned plus one: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
