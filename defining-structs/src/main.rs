fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someUsername124"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotherEmail@example.com");

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherUsername567"),
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _point = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn _build_user(email: String, username: String) -> User {
    User {
        email,    // email: email,
        username, // username: username,
        active: true,
        sign_in_count: 1,
    }
}
