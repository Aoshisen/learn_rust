use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
# 使用panic 还是不用panic
## 当程序 发生错误的时候 你可以立即调用panic! 来让程序恐慌,但是这样的话, 不管这个错误能不能恢复,那么后续的代码都不能处理了,
## 当程序发生错误的时候, 如果返回Result 值的话,我们把处理错误的选择权交给了调用者,而不是代替他们做出决定,他们可以认为这个代码是不可恢复的然后调用panic! 或者会有一些恢复的逻辑,让程序变成可以正常运行的代码
## 适合panic! 的例子
- 实例代码,代码原型,以及测试代码都非常适合panic
- 如果被人调用了你你的代码 并且传递了一个没有意义的值,那么就应该panic,如果 你正在调用一个不受控制的代码,并且他返回了一个无法修复的无效状态,那么我们继续panic 才是最合适的
- 当代码对值进行操作的时候,首先验证这个值是否有效,如果无效,那么就panic,函数的契约 只有输入满足了特定的条件时才能得到正确的输出,那么违反契约的时候panic 是有道理的,函数的契约,应该在函数的api 文档重得到解释
> 虽然 我们有很多的错误很检查,但是正是因为这些严格的检查,我们才能确定的知道我们的代码 的值类型以及可以进行的具体操作,而不用在函数内部 判断其空值或者不存在的情况

## 来为我们的猜数游戏创建一个自定义的类型来验证其输入值的有效性

use rand:Rng;
use std::cop::Ordering;
use std::io;


*/
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}

// 使用我们自定义的新类型来验证 创建的值是否在0-100 中间
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100 got {}.", value)
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
// >Rust 的错误处理功能被设计为帮助你编写更加健壮的代码。panic! 宏代表一个程序无法处理
// 的状态，并停止执行而不是使用无效或不正确的值继续处理。Rust 类型系统的 Result 枚举代
// 表操作可能会在一种可以恢复的情况下失败。可以使用 Result 来告诉代码调用者他需要处理
// 潜在的成功或失败。在适当的场景使用 panic! 和 Result 将会使你的代码在面对不可避免的错
// 误时显得更加可靠。