//枚举与模式匹配
/*
# 强大的控制流运算符match
- 允许一个值与一系列模式进行匹配, 并执行匹配的模式对应的代码
- 模式可以使字面量,通配符,变量名
-NOTE: 绑定值的模式
-匹配的分支可以可以绑定到被匹配对象的部分值
-因此可以从enum 变体中提取值
## 匹配Option枚举

match 匹配必须穷举所有的可能
使用下划线 _ 通配符来列举剩下的 所有未指定的穷举类型
 */
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
fn main() {
    let c = Coin::Quarter(UsState::Alabama);
    let s = value_in_coin(c);
    println!("s {}", s);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_coin(c: Coin) -> u8 {
    match c {
        // 匹配成功的表达式就会作为整个match 表达式的返回值进行返回
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter {:?}!", state);
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
// 使用下滑线 列出穷举的所有可能性
fn match_all(v: u8) -> u8 {
    match v {
        1 => 1,
        3 => 3,
        5 => 5,
        7 => 7,
        _ => 0,
    }
}
