/*
# 集合类型 存储在堆内存上面的
## vector
- 由标准库提供
- 可存储多个值
- 只能存储相同类型的数据
- 值在内存中连续存放
*/
fn main() {
    println!("hello world");
    //创建vector
    let v: Vec<i32> = Vec::new();
    //使用vec! 来创建
    let v2 = vec![1, 2, 3];
    //更新vector
    let mut v3 = Vec::new();
    v3.push(1);
    // 读取vector 内部的元素
    // let third = v2[100];
    // println!("The third element is {}", third);
    match v2.get(100) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    };
    // test();
    // each();
    each1();
}
//删除vector 当 vector 离开作用域之后,他就被清理掉了,它所有的元素也被清理掉了

fn test() {
    // 不能在同一作用域内拥有一个值的可变引用和不可变引用
    let mut v = vec![1, 2, 3, 4, 5];
    // 不可变引用
    let first = &v[0];
    //可变引用
    // v.push(2);
    // 不可变引用
    println!("The first element is {}", first);
}
fn each() {
    //如何遍历vector
    let v = vec![100, 32, 57];
    for i in v {
        println!("{}", i);
    }
}

fn each1() {
    //如何遍历vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in v {
        println!("{}", i);
    }
}
