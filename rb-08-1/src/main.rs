fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(4);
    v.push(2);

    // let second = &v[1];
    // let third = v[2];

    for n in &mut v {
        // let plus_one = *n + 1;
        *n += 12;
        println!("{n}");
    }

    // println!("{} {}", second, third);
}
