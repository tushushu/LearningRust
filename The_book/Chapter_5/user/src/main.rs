use std::string::String;

fn main() {
    let user1 = build_user(String::from("user1"), String::from("user1@example.com"));

    let mut user2 = build_user(String::from("user2"), String::from("user2@example.com"));
    user2.active = false;

    let user3 = User {
        username: String::from("user3"),
        email: String::from("user3@example.com"),
        ..user1
    };
    println!(
        "username: {},\nemail: {},\nsign_in_count: {},\nactive: {}.",
        user3.username, user3.email, user3.sign_in_count, user3.active
    )
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
