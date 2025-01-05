//常量的命名规则
// 1. 常量名使用下划线分隔的大写字母单词
// 2. 常量可以在任何作用域中声明，包括全局作用域
// 3. 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算得到的值
// 4. 常量不能使用 mut 修饰符，因为常量本身就是不可变的
const MAX_POINTS: u32 = 100_000;
// 在 Rust 中，变量默认是不可变的。这意味着一旦值被绑定到一个名称上，你就不能改变这个值。
// 但是，有时候我们需要改变一个值，这时就需要使用 mut 关键字来将变量标记为可变的。

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let mut x = x * 2;
    println!("The value of x is: {}", x);
    x = 3;
    println!("The value of x is: {}", x);
}
