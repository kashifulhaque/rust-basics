struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
// They don't have names associated with the fields, but instead, they have the type associated.
struct Color(u8, u8, u8);
struct Point(i32, i32);

fn main() {
    let new_user = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    let another_user = create_user("abc@xyz.io".to_string(), "mr_abc".to_string());
    let another_one = User {
        email: String::from("anotherone@djkhaled.lol"),
        username: String::from("another_one"),
        active: new_user.active,
        sign_in_count: new_user.sign_in_count,
    };
    let another_way = User {
        email: String::from("anotherway@struct.rs"),
        username: String::from("another_way"),
        ..another_user
    };

    // Tuple struct declaration
    let white = Color(255, 255, 255);
    let origin = Point(0, 0);
}

fn create_user(email: String, username: String) -> User {
    // When the name of the field and the name of the parameter are the same, we can use the following short-hand notation.
    // "email" is the same as "email: email"
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
