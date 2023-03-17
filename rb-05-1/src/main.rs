struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct Point(i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("username123"),
    );
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    user1.username = String::from("anotheremail@example.com");
    println!(
        "{0} {1} {2} {3}",
        user1.username, user2.email, user1.active, user2.sign_in_count
    );
}
