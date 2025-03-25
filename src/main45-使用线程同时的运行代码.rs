use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} for the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} for the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    // 使用当前作用域的值 然后传递到新的线程使用
    let v = vec![1, 2, 3];
    //使用move 关键字把所使用到的值放入新线程
    let handle1 = thread::spawn(move || println!("here's a vector {:?}", v));
    //使用drop() 来在线程调用前清理v ,因为v被移入了新的线程所以在当前主线程没有所有权而不能清理,导致报错 (value used here after move)
    // drop(v);
    handle1.join().unwrap();
}
