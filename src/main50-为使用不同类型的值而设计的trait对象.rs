// 为使用不同类型而设计的trait 对象
// 我们如果需要实现一个gui 库,我们可能需要暴露一些类型,方便用户操作, 不如Button 或者TextField, 但是这些每个图形,必须自己实现一个draw 方法来把自己绘制到屏幕上
// 在**传统的** oop 程序里面,我们可能会需要使用 一个基础的Component,该类上有一个draw 方法, 我们的TextField 和Button 必须基于这个基础的Component 类创建
//我们来看看 Rust 中如何实现的
// 1. 定义通用的行为
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("Ass")],
            }),
            Box::new(Button {
                width: 20,
                height: 10,
                label: String::from("11"),
            }),
        ],
    };
    screen.run();
}

pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// 实现特定的trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("hello world {}", self.height);
    }
}
struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("hello world {}", self.height);
    }
}

// trait 对象执行动态分发,
// trait 对象要求对象安全,
// 1. 返回值不为self
// 2. 方法没有任何泛型类型参数
