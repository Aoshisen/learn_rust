// # 可反驳性 模式匹配是否会匹配失败
// 1. 不可反驳 let x=5;
// 2. 可反驳 if let Some(x)=a_value  如果变量
fn main() {
    println!("hello world!");
    let some_option_value = Some(2);
    //这段代码是不能编译的 编译器会在编译时抱怨 尝试在要求不可反驳模式的地方使用可反驳模式
    // let Some(x) = some_option_value;

    //使用if let (可反驳模式)
    if let Some(x) = some_option_value {}
    //编译器会抱怨将不可反驳模式用于if let 是没有意义的
    // if let x = 5 {
    //     println!("{}", x)
    // }
    // 使用match 分支匹配只有一个值的情况,
    let value = 42;

    // 使用 match 的不可反驳模式（只有一个分支）
    match value {
        x => println!("The value is: {}", x),
    }

    // 这完全等同于简单的 let 绑定： 但是使用let 会更简单
    let x = value;
    println!("The value is: {}", x);
}
