//使用sync 和send 的可扩展并发
//内嵌于语言当中的并发概念 std::marker 中的Sync 和Send trait.

// 通过Send trait 表明所有权可以在线程间传递,几乎所有的Rust 类型都是Send 的,但是Rc<T> 不是,因为克隆了Rc<T>的值并尝试将克隆的所有权转移到另一个线程,这两个线程都可能同事更新引用计数.
//sync 允许多线程访问
// Sync 表明一个实现了Sync 的类型可以安全的在多个线程中拥有其值的引用,对于任意类型 T,如果 &T 是Send 的,那么T 就是Sync 的,这意味着其引用就可以安全的发送到另一个线程.
// 智能指针Rc<T> 也不是Sync 的,出于其不是Send 的相同原因.

//手动实现Sync 和Send Trait 是不安全的,他们只是标记trait,甚至不需要实现任何方法,只是用来加强并发相关的不可变性

fn main() {
    println!("hello world!");
}
