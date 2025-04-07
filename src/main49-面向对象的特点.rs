// rust 的面向对象编程特性  关于oop 是什么有很多相互矛盾的定义,在某些定义下面,rust 是面向对象的,但是在其他定义下,rust 不是
// 如果一个语言必须有继承才能被称为面向对象语言的话,那么rust 就不是面向对象的. 但是rust 提供了其他的解决方案,
// 1. 重用代码,一旦为一个类型实现了特性行为,那么 继承可以对任意不同的类型重用这个实现,可以使用默认实现,然后子类通过覆盖父类继承的方法实现  (比如 )

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub trait SummaryDefault {
    fn summarize(&self) -> String {
        String::from("(Read more ...)")
    }
}
//提供默认实现 但是可以自己定义该方法的实现
impl SummaryDefault for NewArticle {
    fn summarize(&self) -> String {
        String::from("SummaryDefault NewArticle ")
    }
}

fn main() {
    let a = AveragedCollection::new(vec![1, 24]);
    println!("hello world! {}", a.average);
}

//封装隐藏了实现细节
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(value: Vec<i32>) -> AveragedCollection {
        let mut ins = AveragedCollection {
            list: value,
            average: 0.0,
        };
        ins.update_average();
        ins
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average()
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
