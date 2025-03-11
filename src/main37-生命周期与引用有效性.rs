// 生命周期以及其有效性
//## 生命周期避免了悬垂引用
// 如果声明了一个变量但是没有给他赋值,如果在给他赋值前去进行访问,那么会出现一个编译时错误

fn main() {
    println!("hello world!");

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // 	// x 在退出作用域的时候会被清理掉,所以r 如果赋值允许的情况下,就指向了一个错误的地址,或者是一个空的地址,造成了悬垂引用;
    // }
    // println!("r:{}", r)
    //尝试编写一个判断字符串长短的函数,返回字符串长度较长者
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// rust 中的借用检查器,以及生命周期标识
// {
//     let r;                // ---------+-- 'a (r 变量生命周期范围)
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |  (x 变量的生命周期范围)
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+
// r 的生命周期比x 大 , 而r 却引用了 一个比他生命周期小的x 变量,所以编译器 拒绝编译程序. (如果生命周期小,就不能保证在自己生命周期内,引用的生命周期一直有效)

// 函数中的泛型生命周期

//下面这段代码是有问题的,因为编译器 不知道要返回的引用指向的是x或者y,事实上我们也不知道.因为函数体里面一种可能是返回x 的引用,而另一种可能是返回y 的引用
// 我们也没办法观察作用域来确定他们的引用是否总是有效.使用借用检查器也没办法知道,因为他不知道x和y的生命周期如何与返回值的生命周期相关联的
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         return x;
//     }
//     y
// }
// 我们增加 生命周期标注 来帮助rust 编译器了解我们的代码之间 多个引用生命周期的相互影响关系.
// 生命周期的标注不改变任何引用的生命周期的长短. 生命周期的语法是以 ' 开头

// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用
// &'a mut i32 // 带有显式生命周期的可变引用

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}
// 下面一段话解释了 单个生命周期标注为什么没有意义
// 因为生命周期标注告诉rust 多个泛型生命周期如何相互联系的,如果一个函数的两个参数的生命周期都被标注成了'a, 那么这个标注的含义就是这两个参数的生命周期 应该和这个泛型的生命周期一样长
// 为了强化这个记忆, 一个传入了 两个参数的函数 其两个参数的生命周期被显示的标注了,那么现在函数签名表明对于某些生命周期'a ,函数获取了两个参数,他们都是与生命周期'a 存在的一样长的字符串slice. 函数也会返回统一也与生命周期 'a 存在的一样长的字符出slice
// 实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致
// 我们用相同的生命周期'a 标注 x,y ,但是却返回了x,y 生命周期的较小者, 但是小的生命周期中 更大的生命周期 总是有效的 (而我们返回的就是较大的那个生命周期,所以返回的引用值 在实际的返回的 生命周期中总是有效的)

// 直观的例子 (因为生命周期 返回的是 较小的生命周期,那么 小的生命周期总能保证 该生命周期中的 引用都是有效的, ) string1 是全局有效的,而string2 则是在局部有效的,而result 则引用了在局部内部作用域都是有效的值, 这个代码就是有效的
// {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }

//但是下面这个 虽然直觉上是会返回string1 而string1 拥有全局的作用域,这个例子是正确的(但是 代入我们的生命周期规则,这个longest 返回的 两个参数中较短的那个) (另一种解释方法是: 为了保证 result 的有效性,那么就必须保证string1,string2 都必须直到外部作用域结束之前都是有效的,而显然string2 不满足,所以报错)

// {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//     }
//      println!("The longest string is {}", result);
// }

// 尝试 指定生命周期,然后返回一个悬垂引用 (因为返回的生命周期与参数完全没有关系,返回了一个悬垂引用,这个时候考虑返回一个具有所有权的值 而不是 一个值的引用)
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

//结构体中定义生命周期
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//生命周期省略规则
//函数或者方法的参数的生命周期被成为输入生命周期,而返回值的生命周期被称为输出生命周期
//三个规则 前两个补全输入生命周期,后一个补全输出生命周期
// 1. 每一个引用的参数都有它自己的生命周期
// fn foo<'a>(x: &'a i32) {}
// fn foo<'a,'b>(x:&'a i32,y:&'a i32){}
//fn foo<'a, 'b>(x: &'a i32, y: &'b i32){}

// 2. 如果只有一个输入生命周期,那么它被赋予所有输出生命周期
// fn foo<'a>(x: &'a i32)->&'a i32 {}

//3. 如果方法有多个输入生命周期并且其中一个是&self 或者&mut self,那么所有输出生命周期被赋予self 的生命周期.

// 针对于下面这个函数
// fn first_world(s: &str) -> &str {}
// ----- 应用第一条规则---- (每个引用参数都有一个他自己的生命周期)
// fn first_world(s: &'a str) -> &str {}

// -- 应用第二条规则 --- (如果只有一个输入生命周期那么它被赋予所有输出生命周期)

// fn first_world(s: &'a str) -> &'a str {}

// --- 已经补全了生命周期了,而且第三条规则应用不上

// 然后再看我们之前的longest 函数的代码
// fn longest(x: &str, y: &str) -> &str {
// ---- 应用第一条生命周期补全规则
// fn longest(x: &'a str, y: &'b str) -> &str {

//第二条第三条规则应用不上,所以不行

// 对于带有生命周期结构体 为其实现方法时
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

//使用于第三条补全规则的 例子
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 静态生命周期 'static 在整个程序的生命周期里面都有效的,但是 谨慎使用

// 结合泛型类型参数 trait bound 和生命周期的一个函数的例子

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
//     where T: Display
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
