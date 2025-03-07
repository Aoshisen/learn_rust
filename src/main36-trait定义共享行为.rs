use std::fmt::{Debug, Display};

//使用trait 定义共享的行为
fn main() {
    println!("hello world!");

    // let article = NewArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best
    // hockey team in the NHL.",
    //     ),
    // };
    // println!("article {}", article.summarize())
    println!("{:?}", largest(&vec![2, 3, 4]))
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//定义trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// impl Summary for NewArticle {
//     fn summarize(&self) -> String {
//         format!("{},by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}
// 注意: 只有当trait 或者要实现的trait 位于crate 的本地作用域的时候才能为该类型实现trait 可以为自定义的struct Tweet 实现标准库中的Display trait ,或者为标准库中的 struct 实现自定义的trait
// 但是不能为外部 struct(结构体) 实现外部的 trait(正面)

//trait 的默认实现, 如果有默认实现以及有具体的实现,rust 编译器会报出错误;

// pub trait SummaryDefault {
//     fn summarize(&self) -> String {
//         String::from("(Read more ...)")
//     }
// }
// impl SummaryDefault for NewArticle {}

//可以在定义trait 的时候调用已经定义好的方法 作为默认实现,
// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }

// 使用trait 作为参数,就是传递的值必须要满足指定的trait 才能完成调用

// pub fn notify(item: impl Summary) {
//     println!("Breaking news {}", item.summarize());
// }

//trait bound 语法
// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news {}", item.summarize());
// }

//如果有两个需要实现的trait bound 那么就有下面的代码
// pub fn notify(item: impl Summary + Display) {}
pub fn notify<T: Summary + Display>(item: T) {}

//通过where 关键字简化trait bound
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
fn some_function<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

//返回实现了trait 的类型
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course ,as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

//但是下面这个函数就不能正常编译,这等到后面说
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best
//             hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
fn largest<'a, T: PartialOrd>(list: &'a [T]) -> &'a T {
    //使用生命周期标定 largest 的生命周期
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list.iter() {
//         if item > largest {
//             largest = &item;
//         }
//     }
//     largest
// }

// //改造largest 方法
// fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
//     list.iter()
//         .max_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap()
// }
