fn main() {
    let res = assign_loop_value();
    let res2 = labeled_loops(10, 5);
    let a = [10, 20, 30, 40, 50];

    // Testing ranges and the rev function on ranges.
    for number in (1..4).rev() {
        println!("{number}");
    }

    loop_an_array(a);
    println!("The result is {res}");
    println!("End count = {res2}");
}

fn assign_loop_value() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    result
}

fn labeled_loops(rem: i32, until: i32) -> i32 {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = rem;

        loop {
            println!("remaining = {remaining}");
            if remaining == until {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    count
}

fn loop_an_array(ar: [i32; 5]) {
    for element in ar {
        println!("the value is: {element}");
    }
}
