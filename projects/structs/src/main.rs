struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(String::from("sample@example.com"), String::from("peko"));

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user1: {}", user1.active)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
