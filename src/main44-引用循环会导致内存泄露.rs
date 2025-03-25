use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
//将Rc<T> 变成Weak<T> 来避免循环引用;
// 使用Rc::clone() 会增加Rc<T> 实例的strong_count,和值在其strong_count 为0 才会被清理的Rc<T> 实例;
//使用Rc::downgrade 并传递Rc<T> 实例的引用来创建弱引用(weak reference)

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    // 增加父 的引用
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        //创建一个循环引用
        // a 被b 所引用
        // b 又被接入到 a的tail() RefCell::new(Rc::new(Nil)) 所以他是可变的
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    //使用弱引用;
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
