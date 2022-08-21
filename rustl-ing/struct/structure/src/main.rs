struct Color(u8, u8, u8);
struct Point(f64, f64, f64);

// unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotherexample.com");

    // too long
    let user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another2@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    // use this
    let user3 = User {
        email: String::from("user3@email.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
