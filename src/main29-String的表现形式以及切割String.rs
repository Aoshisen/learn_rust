/*
# String 的形式和切割
- 内部表示
- String 实际上是Vec<u8> 的包装
- String Unicode 标量值
- 字节, 标量值, 字形簇
# 字符串的 切割
*/
fn main() {
    println!("hello world");
    each();
}
fn slice() {
    let s = String::from("hello ");
    let s2 = &s[..2];
    //但是如果不是字符串的边界,那么程序就会恐慌
}

fn each() {
    let s = String::from("hello ");
    for i in s.bytes() {
        println!("bytes {}", i);
    }

    for i in s.chars() {
        println!("bytes {}", i);
    }
}
