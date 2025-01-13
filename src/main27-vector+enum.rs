/*
# 使用enum来存储多种数据类型
- enum 的变体可以附加不同类型的数据
- enum 的变体定义在同一个enum 系统下

*/

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    println!("hello world!");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(1.2),
    ];
}
