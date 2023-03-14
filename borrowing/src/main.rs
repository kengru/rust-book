fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{s}");

    // Checking dereferencing.
    let x = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);

    vec_mut();

    // Checking permissions.
    let mut v = vec!['h', 'e', 'l', 'l', 'o'];
    let vv = &mut v;
    ascii_capitalize(vv);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

fn vec_mut() {
    let mut vect = vec![1, 2, 3];
    let num = &mut vect[2];

    *num += 1;
    println!("Third element is {}", *num);

    println!("Vector is now {:?}", vect);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
