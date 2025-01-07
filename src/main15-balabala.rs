#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
}
fn main() {
    let rect = Rectangle {
        width: 12,
        height: 12,
    };
    println!("area is {}", rect.area());
}

// 在rust 中方法和函数不一样
//方法是定义在struct enum 或者是 trait 对象 的上下文中定义
// 方法的第一个参数是self,标识方法调用的struct 实例

//关联函数 比如String::from
//可以在impl 里面定义不使用&self 作为第一个参数的函数,叫做关联函数
//关联函数通常用于构造器 
// example
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
