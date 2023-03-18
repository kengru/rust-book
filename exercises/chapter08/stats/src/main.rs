//! Given a list of integers, use a vector and return the median
//! (when sorted, the value in the middle position) and mode
//! (the value that occurs most often; a hash map will be helpful here)
//! of the list.

use std::collections::HashMap;

fn main() {
    let mut list = vec![1, 2, 6, 4, 9, 12, 3, 5, 2, 44, 2, 3];
    let mut map = HashMap::new();
    let median: i32;
    let mut mode: i32 = 0;

    list.sort();

    let n = list.len();
    let even = list.len() % 2 == 0;
    median = if even {
        (list[(n / 2) - 1] + list[(n / 2)]) / 2
    } else {
        list[n / 2]
    };

    for val in list {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    for val in map {
        if val.1 > max {
            max = val.1;
            mode = val.0;
        }
    }

    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
