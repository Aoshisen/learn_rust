// 使用Rc<T>引用记数智能指针;
// 有一个List 然后两个新的List 会拥有之前的这个List
//Rc<T> 只能用于单线程
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
//     // 这个代码会报错,因为a 已经移入了b中,所以b就拥有了a,当再次使用a 创建c时,因为所有权移动了,所以就不被允许;
// }

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //RC::clone 和 a.clone() 的区别,RC::Clone 不会进行深拷贝,只会增加引用计数,a.clone() 则会进行数据的深拷贝,可能会花费很长时间;
    //1
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    //2
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        //3
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    //2
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
}
