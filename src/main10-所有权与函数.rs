//所有权与函数
/*
## 所有权与函数
- 在语义上把值传递给函数与把值赋值给变量是类似的
- 将值传递给函数 要么值会发生移动要么会发生复制
*/
fn main() {
    // println!("所有权与函数");
    // let s = String::from("Hello World");
    // // NOTE: 这和常规的js 不一样; 有很大的不一样
    // // s 的所有权被移动到了函数内部,在执行完函数后的下一行代码,s 变量就不会再有效了,s 发生了移动
    // take_ownership(s);
    // // println!("s is {}", s);
    // let x = 5;
    // // NOTE: 由于 x 是i32 类型的,其实现了copy 的trait ,那么传入函数的就是一个副本 ,那么在当前的作用域范围内,x 在调用了makes_copy x 的所有权还是在当前的环境中,所以还是可以正常的访问x
    // makes_copy(x);
    // println!("x is {}", x);
    test()
}
fn take_ownership(some_string: String) {
    // 因为这里函数传递进来了一个位于堆上面的String 值,那么在退出作用域的时候这个some_string 的所有权就会被移动到函数内部,那么调用完函数之后这个s 就不会再有效了
    println!("some_string {}", some_string);
    //在离开作用域的时候some_string 就会去调用drop 函数 释放掉some_string 对应的内存,
}
fn makes_copy(x: i32) {
    println!(" x is {}", x);
    //对于这种数据类型来说,不会有什么特别的事情发生,因为是创建的是标量类型的副本
}

/*
## 返回值与作用域,函数的返回值也可以发生所有权的转移
- 一个变量的所有权总是遵循同样的模式
- 当一个值赋值给其他变量的时候变量的所有权也会发生转移
- 当一个包含heap 数据的变量离开作用域时 ,就会调用drop 清理掉该值锁占用的堆内存,除非这个变量的所有权交给了另一个变量
*/
fn test() {
    let s1 = gives_ownership();
    let s2 = String::from("Hello world!");
    let s3 = take_and_gives_back(s2);
    //在这里s2 由于所有权已经移交给了take_and_gives_back s2 的所有权已经没有在该环境了,所以这里会有问题
    // println!("s2{}",s2);
    println!("s3{}", s3);
}
fn gives_ownership() -> String {
    let s = String::from("S1 back");
    s
}
fn take_and_gives_back(s: String) -> String {
    s
}
/*
让函数使用值,且不获得其所有权
*/
fn test1() {
    let s1 = String::from("Hello");
	//
    let (s2, size) = calc_size(s1);
    println(" {} 's length is {}", s2, size)
}
fn calc_size(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
