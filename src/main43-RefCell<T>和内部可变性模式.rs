// #  RefCell<T> 和内部可变性
// 想象一下 你有 三个list  一个基础的list 两个依赖这个基础list 的 list 现在想要修改这个基础list 里面的一个值,该怎么办呢,这个时候就要使用到了RefCell<T> ,内部可变
// 内部可变性是Rust 中的一个设计模式,允许即使在有不可变引用的时候也可以改变数据.
// 在实际的代码结构中如何选择 Box<T>,Rc<T>,RefCell<T>
//- Rc<T> 允许数据有多个所有者; 而Box<T>,RefCell<T>只能有一个所有者;
//- Box<T> 允许在编译时执行不可变或者可变借用检查; Rc<T>仅允许在编译时执行不可变借用检查;RefCell<T> 允许在运行时执行不可变或者可变检查;
// RefCell<T> 的可变或者不可变检查发生在运行时,所以我们就可以在即便RefCell<T> 自身不可变的情况下修改其内部的值;

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // value 和当前list ,因为当前值需要支持有多个所有者,所以使用Rc<T> 引用计数器,又因为我们使用值的内部可变性,所以使用RefCell<T> 来包裹i32
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
fn main() {
    println!("hello world!");

    //NOTICE: 我们不可以可变的借用 一个不可变值
    // let x=5;
    // let y =&mut x;
    // 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
