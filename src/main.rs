use std::{thread, time::Duration};
//# 闭包: 可以捕获环境的匿名函数;
// rust 中的闭包可以保存进变量,可以作为参数传递给其他函数的匿名函数, 我们可以在一个地方创建闭包,在不同的上下文执行闭包运算.
// 不同于函数,闭包可以捕获调用者作用域中的值
// 可以更好的组织代码和自定义行为
// 昂贵的计算 rust 后端代码 生成健身计划
fn main() {
    // 需要两个变量
    //1. 来自用户的intensity 数字,代表用户喜欢的计划的强度等级
    //2. 一个随机数,在健身计划中生成变化.
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number)
}
// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         //如果强度小于25
//         //俯卧撑
//         println!(
//             "Today, do {} pushups",
//             simulated_expensive_calculation(intensity)
//         );
//         //仰卧起坐
//         println!(
//             "Next, do {} situps",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         //如果强度大于25
//         if random_number == 3 {
//             // 如果随机数等于3 那么
//             println!("Take a break tody! Remember to stay hydrated!");
//         } else {
//             //随机数不等于3
//             //跑步多少分钟
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             )
//         }
//     }
// }

//使用一个值来保存我们昂贵计算所计算出来的结果;(但是这样会存在一种情况就是不需要这个昂贵计算的时候也会去计算其值)
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_result = simulated_expensive_calculation(intensity);
//     if intensity < 25 {
//         //如果强度小于25
//         //俯卧撑
//         println!("Today, do {} pushups", expensive_result);
//         //仰卧起坐
//         println!("Next, do {} situps", expensive_result);
//     } else {
//         //如果强度大于25
//         if random_number == 3 {
//             // 如果随机数等于3 那么
//             println!("Take a break tody! Remember to stay hydrated!");
//         } else {
//             //随机数不等于3
//             //跑步多少分钟
//             println!("Today, run for {} minutes!", expensive_result)
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     //闭包的定义 出现在闭包的变量名后的=后面就是闭包的定义 闭包用|开始,然后里面是闭包的参数,如果有多个参数以 , 分割 |a,b|{} 大括号里面就是闭包里面需要执行的逻辑,成为闭包体,如果闭包体里面只有一行代码的话,那么闭包体的括号是可以省略的,
//     // 在闭包的末尾 使用分号 让let 语句完整,
//     //使用闭包替换
//     let expensive_closure = |num| {
//         println!("calculating slowly ...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//     if intensity < 25 {
//         //如果强度小于25
//         //俯卧撑
//         println!("Today, do {} pushups", expensive_closure(intensity));
//         //仰卧起坐
//         println!("Next, do {} situps", expensive_closure(intensity));
//     } else {
//         //如果强度大于25
//         if random_number == 3 {
//             // 如果随机数等于3 那么
//             println!("Take a break tody! Remember to stay hydrated!");
//         } else {
//             //随机数不等于3
//             //跑步多少分钟
//             println!("Today, run for {} minutes!", expensive_closure(intensity))
//         }
//     }
// }
// 关于闭包的类型推断和标注,闭包不需要像fn 函数那样进行函数参数和返回值的注明类型,(因为闭包通常很短小且类型都可以能靠编译器推断出来)
// 如果对一个闭包传入两个不同类型的值,那么编译器会得到一个类型错误,因为当第一次调用闭包的时候,传入的值的类型就被锁定到了闭包内部,不允许更改了;

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;
//回到我们的generate_workout 函数,虽然说使用了闭包但是这个闭包的函数还是在很多地方重复的调用了,有两种思路,在全局的generate_workout 内部定义一个值用来接受我们昂贵计算的函数的返回值,如果有那么就不再调用昂贵花费的函数了,但是这种方法很重复,也不好维护
// 我们可以定义一个 存放闭包和调用闭包的结构体,该结构体只会在需要结果的时候执行闭包,然后返回闭包执行的值,并且会缓存其值.
// 为了让结构体能够储存闭包,所以我们需要指定闭包的类型,因为结构体定义知道每一个字段的类型,每一个闭包实例都有自己独有的匿名类型, 即便两个闭包有着相同的签名,但是他们的类型仍然可以被认为是不同的.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

//现在cacher 实现的限制,如果有值的话,就不执行具体的代码了,并且一旦有值,无论参数怎么变化,都只会返回之前已经缓存了的值
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        //如果强度小于25
        //俯卧撑
        println!("Today, do {} pushups", expensive_result.value(intensity));
        //仰卧起坐
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else {
        //如果强度大于25
        if random_number == 3 {
            // 如果随机数等于3 那么
            println!("Take a break tody! Remember to stay hydrated!");
        } else {
            //随机数不等于3
            //跑步多少分钟
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
//使用闭包重构复杂计算逻辑

// //闭包使用三种方式来捕获环境中的变量,分别是
// - FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
// - FnMut 获取可变的借用值所以可以改变其环境
// - Fn 从其环境获取不可变的借用值

//关系是,所有的闭包都实现了FnOnce,没有移动变量所有权的闭包实现了FnMut,不需要对变量进行可变访问的闭包实现了Fn,所以 FnMut>Fn>FnOnce;
// 使用move 移动变量所有权到闭包内部, 当变量被移动到闭包内部的时候,闭包获取了变量的所有权,那么在闭包的上层就不能访问 该移动了的变量了; 技巧: 在使用的时候使用Fn 的trait bound 然后编译器会告诉你是否需要FnMut 或者是FnOnce;
