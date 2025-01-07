//敲代码实现 一个长方形的面积计算
fn main() {
    let width: u32 = 30;
    let height: u32 = 50;

    let rect = Rectangle { width, height };
    println!("area is {} ", area(&rect));
    // struct 不是基础类型,就没有实现Display trait
    // println!("{}", rect)
    println!("{:?}", rect)
    println!("{:#?}", rect)
}
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 但是这个方法的语义化也不够好
// struct Rectangle(u32, u32);
// fn area(dim: Rectangle) -> u32 {
//     dim.0 * dim.1
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
