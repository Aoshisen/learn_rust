//使用发消息的机制 来在不同的线程之间通信,类似于iframe 通信只能通过发消息 rust 受到golang 语言启发,不要通过共享内存来通讯,而是通过通讯来共享内存;
// 信道通过两部分组成,发送者(transmitter),接受者(receiver),当发送者或接受者任意一方被丢弃时,就可以认为信道被关闭了;
//mpsc 是多个生产者,单个消费者的意思(multiple producer ,single consumer)
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    println!("hello world!");
    let (tx, rx) = mpsc::channel();
    //使用clone 来创建多个发送者
    let tx1 = tx.clone();

    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        //在send 完成之后使用该值是不允许的
        // println!("val is {val}")
        let v = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in v {
            //这里需要用到tx 所以需要使用move 关键字将tx 移入该线程
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            //这里需要用到tx1 所以需要使用move 关键字将tx 移入该线程
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    // //接受者有两个方法recv()[会阻塞主线程执行,直到有值的时候,会在一个Result<T,E> 中返回他,但是 如果信道关闭,recv 会返回一个错误来表明不再有新的值进来了]
    // //try_recv() 不会阻塞,但是他会立即返回一个Result<T,E> OK 值包含可用的信息,Err 表示没有任何消息,可以通过循环来接受消息,在有可用消息的时候处理消息,其余时候可以处理其他工作,直到再次检查
    // println!("Go {}", received);
    //使用循环打印输出
    for received in rx {
        println!("Got {received}");
    }
}
