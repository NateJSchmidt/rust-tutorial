#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(
        String::from("someone@noreply.com"), String::from("someone"));

    println!("User 1: {:?}", user1);
    println!("User 2: {:?}", user2);

    let user3 = User {
        email: String::from("NewUser"),
        ..user1
    };

    println!("User 3: {:?}", user3);
    // the following is not allowed due to ownership on the username field
    // println!("User 1: {:?}", user1);
    // but we can still access the email field
    println!("User 1 email: {}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
