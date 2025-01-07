/*
# 定义枚举
*/

enum IPAddKind {
    V4,
    V6,
}
// struct IpAddr {
//     kind: IPAddKind,
//     address: String,
// }

// 在rust 中可以允许数据直接附加在类型的变体中 比如
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
// 这种结构的好处是不需要额外的struct
// 每个变体可以拥有不同的类型以及关联的量
// 可以在枚举类型的结构体内定义 嵌入任何类型的数据

fn main() {
    let four = IPAddKind::V4;
    let six: IPAddKind::V6;

    // let home = IpAddr {
    //     kind: IPAddKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let lookback = IpAddr {
    //     kind: IpAddr::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddr::V4(127, 0, 0, 1);
    let lookback = IpAddr::V6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 12 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(12, 12, 12);
}

fn rout(s: IPAddKind) -> IPAddKind {
    s
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 为枚举实现方法
impl Message {
    fn call(&self) {}
}
