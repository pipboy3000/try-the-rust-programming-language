enum IpAddrKind {
    V4,
    V6
}

// enum IpAddr {
//     V4(String),
//     V6(String)
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// impl Message {
//     fn call(&self) {

//     }
// }

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _roopback = IpAddr::V6(String::from("::1"));

    // let m = Message::Write(String::from("hello"));
    // m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
