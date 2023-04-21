fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {

}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum IpAddrKind {
    V4,
    V6
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // do the work
    }
}

// enum Option<T> {
//     None,
//     Some(T)
// }
