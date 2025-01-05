//控制流程
fn main() {
    println!("Hello world");
    let x: i32 = 8;
    compare(x);
}
// fn compare(x: i32) -> () {
//     // if 后面的类型必须是一个bool 类型,不能是其他类型,不能像javascript 一样,自动转换
//     if x % 4 == 0 {
//         //分支 arm
//         println!("x is divisible by 4");
//     } else if x % 3 == 0 {
//         println!("x is divisible by 3");
//     } else if x % 2 == 0 {
//         println!("x is divisible by 2");
//     } else {
//         println!("x is not divisible by 4, 3, or 2");
//     }
//     // 如果一个值能被 4整除那么其肯定也能被2 整除,由于代码执行是从上到下执行的,所以只会打印第一个分支
// }
// 如果代码里面使用了多余一个的else if 那么代码看上去就会比较凌乱,这时候可以使用match 重构一些
fn compare(x: i32) {
    match (x % 4, x % 3, x % 2) {
        (0, _, _) => println!("x is divisible by 4"),
        (_, 0, _) => println!("x is divisible by 3"),
        (_, _, 0) => println!("x is divisible by 2"),
        _ => println!("x is not divisible by 4, 3, or 2"),
    }
}

fn test() {
    let condition = true;
    // 如果类型不兼容 那么 rust 会报错
    // let number = if condition { 5 } else { "6" };
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);
    // if 是一个表达式,所以可以赋值给一个变量
}
