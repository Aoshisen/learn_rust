// 使用Drop trait 清理代码
// 允许我们在值离开作用域的时候执行一些代码,比如释放文件资源或者网络连接资源,Box<T> 实现了用来释放box所指向的堆空间

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data)
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // 但是我们不能调用 结构体上的方法清理,会得到编译错误
    // c.drop();

    println!("CustomSmartPointers created");
    //默认在这里会先释放other stuff 然后再释放my stuff;

    //如果指定了释放的顺序,那么就会顺序释放,使用drop 不会导致双重释放的错误
    drop(c);
    drop(d);
}
