use std::ops::Deref;
fn main() {
    println!("hello world!");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    //需要为MyBox 实现Deref trait
    assert_eq!(5, *y);
    let s = Box::new(String::from("Ass"));
    hello(&s);
    //如果没有实现解引用的强制转换
    // (*m) 将 MyBox<String> 解引用为 String。接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名
    hello(&(*s)[..]);
}
// 函数和方法的隐式解引用强制转换;
fn hello(t: &str) {
    println!("hello {}", t);
}

//自定义一个自己的MyBox
//是一个结构体
struct MyBox<T>(T);

//实现MyBox 有一个关联函数
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
//通过实现Deref trait 将某类型像引用一样处理

// rust 在发现类型和trait 的实现满足下面三种情况会自动进行解引用强制转换
// 当 T: Deref<Target=U> ：从 &T 到 &U。
// 当 T: DerefMut<Target=U> ：从 &mut T 到 &mut U。
// 当 T: Deref<Target=U> ：从 &mut T 到 &U。
