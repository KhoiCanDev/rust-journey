struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    //tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //unit like struct
    //when need to implement a trait on some type but don't have any data that you want to store in the type
    let subject = AlwaysEqual;
}
