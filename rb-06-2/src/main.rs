#[derive(Debug)]
enum UsState {
    // Alabama,
    // Alaska,
    California,
}

enum Coin {
    // Penny,
    // Nickel,
    // Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        // Coin::Penny => 1,
        // Coin::Nickel => 5,
        // Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {
    let c = Coin::Quarter(UsState::California);
    let value = value_in_cents(&c);

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    println!("And the value is: {value}");
}
