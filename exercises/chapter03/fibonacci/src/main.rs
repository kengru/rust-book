fn main() {
    let position = 10;
    let number = fibonacci_nth(position);

    println!("The {position}th Fibonacci number is: {number}.");
}

fn fibonacci_nth(nth: i32) -> i32 {
    let mut x = 0;
    let mut y = 1;
    for _ in 0..nth {
        let xx = x;
        x = x + y;
        y = xx;
    }
    x
}
