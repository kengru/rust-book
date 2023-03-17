#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}
