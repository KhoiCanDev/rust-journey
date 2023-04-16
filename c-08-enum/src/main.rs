enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Define enum and its attached struct
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Define method for enum
impl Message {
    fn call(&self) {}
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // The usage of Option
    // Some means the variable has value
    // None means the value is absent
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}
