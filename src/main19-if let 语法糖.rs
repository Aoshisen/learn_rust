/*
# if let
- 处理值关心一种情况而忽略其他情况的匹配模式
- 好处是 更少的代码 更少的缩进,更少的模板代码
- 坏处是放弃了穷举的可能
*/
fn main() {
    println!("hello world!");
    let v: Option<u8> = Some(2u8);
    match v {
        //我现在想要只针对v=3 的情况做一些处理
        Some(3) => println!("three"),
        _ => println!("others"),
    };
    // 我们就可以使用 if let 语法糖
    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}
