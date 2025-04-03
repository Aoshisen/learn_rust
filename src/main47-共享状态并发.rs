use std::cell::RefCell;
use std::rc::Rc;
//共享状态并发
// 和在线程间传递消息不同,这个是使用共享内存的方式来在不同的线程间进行并发
// 消息传递管理值的所有权容易一些,而共享内存的方式类似于多所有权,而rust 通过Rc<T> 可以实现多所有权,使共享内存成为可能,但是在多所有权中有个绕不开的话题就是 需要管理这些不同的所有者,这就是互斥器;
// 互斥器一次只允许一个线程访问数据
//互斥器以难以使用著称
// - 在使用数据之前尝试获取锁
// - 在处理完互斥器所保护的数据之后,必须解锁数据,这样其他线程才能获得锁
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// fn main() {
//     // 使用Mutex<T> 互斥器的简单例子
//     // let m = Mutex::new(5);
//     // // println!("hello world!");
//     // {
//     //     //MutexGuard 是一个智能指针 实现了Deref 方法 可以通过 解引用的方式来拿到值,实现了Drop 方法来在离开作用域的时候释放掉持有的锁
//     //     let mut num = m.lock().unwrap();
//     //     *num = 6
//     // }
//     // println!("m={:?}", m)

//     // ####--------- 在线程间共享Mutex<T>
//     // 我们在使用线程的时候会把counter 移动到线程内部,所以在外面就不能使用counter 这个值了
//     // let counter = Mutex::new(0);
//     // let mut handles = vec![];
//     // for _ in 0..10 {
//     //     let handle = thread::spawn(move || {
//     //         let mut num = counter.lock().unwrap();
//     //         *num += 1;
//     //     });
//     //     handles.push(handle);
//     // }
//     // for handle in handles {
//     //     //等待执行
//     //     handle.join().unwrap()
//     // }
//     // // 我们在使用线程的时候会把counter 移动到线程内部,所以在外面就不能使用counter 这个值了
//     // println!("Result {}", *counter.lock().unwrap())
//     //### ----------------- 使用多所有权来修复多线程共享Mutex<T> 的问题
//     // let counter = Rc::new(Mutex::new(0));
//     // let mut handles = vec![];
//     // for _ in 0..10 {
//     //     let counter = Rc::clone(&counter);
//     //     //这里报错说Rc<T> 不能在线程间安全的传递,所以这里有问题
//     //     let handle = thread::spawn(move || {
//     //         let mut num = counter.lock().unwrap();
//     //         *num += 1;
//     //     });
//     //     handles.push(handle);
//     // }
//     // for handle in handles {
//     //     //等待执行
//     //     handle.join().unwrap()
//     // }
//     // println!("Result {}", *counter.lock().unwrap())

//     // 使用实现了send 和sync 的trait Arc<T> 在线程间安全的传递值 (Atomically Reference counted) 和Rc<T>
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         //这里报错说Rc<T> 不能在线程间安全的传递,所以这里有问题
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         //等待执行
//         handle.join().unwrap()
//     }
//     println!("Result {}", *counter.lock().unwrap())
//     // 值得注意的是 Mutex<T> 提供了类似于RefCell 一样的内部可变性,使得可以使用Mutex<T> 来改变Arc<T> 中的内容,rust 不能避免使用Mutex<T> 中的逻辑错误,比如在Rc<T> 中的循环引用错误,同样的,Mutex<T> 也有造成死锁的风险,在一个操作中需要锁住两个资源,而两个资源各持有一个锁,这就会造成他们永远的相互等待

// }

//下面是一个死锁的实例,线程1  持有lock1,然后尝试获取lock2,线程2 持有lock2,尝试获取lock1,然后导致了两个线程互相等待,造成死锁;
fn main() {
    // 创建两个互斥锁，并用 Arc 包装以便多线程共享
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    // 创建第一个锁的引用计数克隆
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);

    // 线程1：先获取 lock1，然后尝试获取 lock2
    let thread1 = thread::spawn(move || {
        println!("线程1 尝试获取 lock1");
        let _guard1 = lock1_clone.lock().unwrap();
        println!("线程1 获取了 lock1");

        // 让线程1稍微睡眠一下，增加死锁概率
        thread::sleep(Duration::from_millis(100));

        println!("线程1 尝试获取 lock2");
        let _guard2 = lock2_clone.lock().unwrap();
        println!("线程1 获取了 lock2");

        println!("线程1 完成了工作");
    });

    // 线程2：先获取 lock2，然后尝试获取 lock1
    let thread2 = thread::spawn(move || {
        println!("线程2 尝试获取 lock2");
        let _guard2 = lock2.lock().unwrap();
        println!("线程2 获取了 lock2");

        // 让线程2稍微睡眠一下，增加死锁概率
        thread::sleep(Duration::from_millis(100));

        println!("线程2 尝试获取 lock1");
        let _guard1 = lock1.lock().unwrap();
        println!("线程2 获取了 lock1");

        println!("线程2 完成了工作");
    });

    // 等待线程结束
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("程序结束"); // 这一行永远不会执行
}
