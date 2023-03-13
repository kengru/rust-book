fn main() {
    for day in 1..13 {
        println!("On the {day}th day of Christmas,");
        println!("my true love sent to me");
        if day > 11 {
            println!("Twelve drummers drumming");
        }
        if day > 10 {
            println!("Eleven pipers piping");
        }
        if day > 9 {
            println!("Ten lords a-leaping");
        }
        if day > 8 {
            println!("Nine ladies dancing");
        }
        if day > 7 {
            println!("Eight maids a-milking");
        }
        if day > 6 {
            println!("Seven swans a-swimming");
        }
        if day > 5 {
            println!("Six geese a-laying");
        }
        if day > 4 {
            println!("Five golden rings");
        }
        if day > 3 {
            println!("Four calling birds");
        }
        if day > 2 {
            println!("Three french hens");
        }
        if day > 1 {
            println!("Two turtle doves, and");
        }
        println!("A partridge in a pear tree");
        println!("");
    }
}
