fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1.@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        active: false,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 2,
    };

    println!("User 1: {:?}", user1);
    println!("User 2: {:?}", user2);

    // Creating a new user with default values
    let mut user3 = User { //mutable
        active: false,
        ..user1 // using user1 as default values
    };
    println!("User 3: {:?}", user3);
    // Creating a new user with default values
    user3.username = String::from("user3");

    println!("User3 name: {}", user3.username);
    println!("User3 email: {}", user3.email);
    println!("User3 sign in count: {}", user3.sign_in_count);
    println!("User3 active: {}", user3.active);

    let user4 : User = build_user(String::from("user@mail.com"), String::from("user4"));
    println!("User 4: {:?}", user4);


    tuple_struct();
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand for username: username
        email, // shorthand for email: email
        sign_in_count: 1,
    }
}


// Structs can also be used to create tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: {:?}", origin);
}