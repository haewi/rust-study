enum IpAddrKind {
    V4, 
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr_e1 {
    V4(String),
    V6(String),
}

enum IpAddr_e2 {
    V4(u8, u8, u8, u8), // different types & amount of associated data is possible
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y:i32},       // anonymous struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // three i32 values
}

impl Message {
    fn call(&self) {} // method of the enum Message
}

fn main() {
    let four = IpAddrKind::V4; // variants of the enum are namespaced under its identifier
    let six = IpAddrKind::V6;

    route(four);
    route(six); // works for both (same type)

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr_e1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr_e1::V6(String::from("::1")); // a concise way

    let home = IpAddr_e2::V4(127, 0, 0, 1);
    let loopback = IpAddr_e2::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");

    // None is similar to Null; invalid value
    let absent_number: Option<i32> = None; // Rust needs to know which type of Option<T> it is

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;  // error can't add i8 and Option<i8>
}

fn route(ip_kind: IpAddrKind) {}