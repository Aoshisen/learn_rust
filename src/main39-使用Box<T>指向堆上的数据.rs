
// # 使用Box<T> 指向堆上的数据
// 最简单最直接的智能指针就是Box<T>, 他允许你将一个值放在堆(heap)上而不是栈上(stack),留在栈上的则是指向堆上的指针
// 除了被储存的位置不同之外,box 没有性能损失, 被用于下面的一些场景
// 当编译未知大小的类型的时候,而又想在上下文中知道确切大小的时候(因为指针的大小是确定的,这是一个存放策略的问题);
// 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权(转移的是指针的值,而不需要对数据进行复制操作,这样迁移数据就很快);
// 当希望拥有一个值,但是只关心它的类型是否实现了特定的trait 而不是其具体类型
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    // println!("hello world!");
    let b = Box::new(5);
    //除了存储位置在堆上以外没有任何区别了
    println!("b={}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list is{:?}", list);
    match &list {
        Cons(val, _next) => {
            println!("head is {}", val);
        }
        Nil => println!("Empty list"),
    }
    //尝试编译会得到infinite 的错误
}
//Box 允许创建递归类型(递归的只是一个指针(指针占据固定大小,如果是具体的数据结构那么会导致rust 不知道为其分配多大的空间,因为推断不出来递归类型的大小));
//ConsList lisp 概念, 一个结构体,包含一个值 以及他本身类型的一个类型
// rust 如何确定一个枚举类型的大小的,在rust 中枚举在同一时刻只存在一种可能,那么其大小就是最大成员所需要的空间
// 但是如果一个成员是递归的类型, 比如Cons我们会计算i32 类型加上一个Cons Cons 类型中包含了i32 类型和Cons 类型的大小,这样的计算是无限的,所以计算出来的大小也是无限的,不知道 具体的值;

//智能指针都实现了Deref trait 以及Drop trait
//Deref 允许Box<T> 被当作引用对待
//Drop 允许Box<T>在离开作用域时 指向堆上的数据被清理

