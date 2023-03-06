// 01
// fn main() {
//     // will print error because s1 is invalidated after assigned to s2
//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     // println!("{s1}, world!");

//     // will work ok
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {s1}, s2 = {s2}");
// }

// 02
// fn main() {
//     let s = String::from("hello"); // s comes into scope
//     takes_ownership(s); // s's value moves into the function ... and so is no longer valid here
//     let x = 5; // x comes into scope
//     makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
// } // Here, x goes out of scope, then s. However, becauses's value was moved, nothing special happens.

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// 03
// fn main() {
//     let s1 = gives_ownership(); // gives_ownership moves its return value into s1
//     let s2 = String::from("hello"); // s2 comes into scope
//     let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {
//     // gives_ownership will move its return value into the function that calls it
//     let some_string = String::from("yours"); // some_string comes into scope
//     some_string // some_string is returned and// moves out to the calling function
// }

// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String {
//     //a_string comes into scope
//     a_string // a_string is returned and moves out to the calling function
// }

// 04
// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1); // s1 ownership to the fn and then return to s2
//     println!("The length of '{s2}' is {len}.");
// }
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a
//     String(s, length)
// }

// 05
// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1); // pass s1 to fn as a reference, the action of creating a reference is borrowing
//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     // s.push_str(", world"); // won't work because can't change borrowing value
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what it refers to, the String is not dropped.

// 06
// change borrowing value by using &mut (mutable reference)
// if you have a mutable reference to a value, you can have no other references to that value
// cannot have a mutable reference while we have an immutable one to the same value
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// 07
// slice
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
