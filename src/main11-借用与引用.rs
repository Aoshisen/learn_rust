/*
# 引用与借用
- 引用其实就是一个值的指针的指针,一个存在于堆上面的数据的指针包含三个内容ptr,len,capacity, 而引用的引用只有一个ptr(就是一个很单纯的引用);
- 而使用引用作为函数参数的行为就要做借用
- 可以使用可变引用对传入的值的引用进行更改,从而改变原有的值
## 可变引用
- 在同一 特定区域内,堆某块区域的可变引用只能,最多不超过一个
- 这样做的好处就是在编译的阶段就可以防止数据竞争
    - 两个或者多个指针访问同一数据
    - 至少有一个指针用于写入数据
    - 没有使用任何机制来同步对数据的访问
- 我们可以通过创建新的作用域来 允许非同时的创建多个可变引用

- 不可以同时拥有可变引用和不可变引用;
- 可以同时拥有多个不可变引用

## 悬空引用 dangling reference
- 悬空指针: 一个指针指向了 内存中的某个地址,而这块内存被释放或者已经分配给了其他人使用了,这时候这个指针就变成了悬空指针
- rust 编译器可以保证所有的指针都不是悬空指针
   - 如果你引用了某个数据,那么rust 将保证 引用没离开作用域之前,数据也不会离开作用域,就是引用和数据同时存在同时释放
*/
fn main() {
    let mut s1 = String::from("Hello");
    let len = calc_len(&mut s1);

    println!("The length of {} is {}", s1, len);
    // let s = dangle();
}
fn calc_len(s: &mut String) -> usize {
    //而我们尝试一下能否修改引用的数据的值呢,不可变引用是不行的
    s.push_str("world");
    s.len()
}

fn multi_reference() {
    let mut s = String::from("hello");
    {
        let s1 = &mut s;
    }
    let s2 = &mut s;
}

// fn dangle() -> &String {
//     let s = String::from("hello");
// 	// 因为 s 在出作用域之后会被回收当前值的所有权,那么对其值的引用就会变成一个悬空引用,在这里编译器会报错
//     //  | fn dangle() -> &String {
//     //    |                ^ expected named lifetime parameter
//     &s
// }
