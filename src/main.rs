use std::collections::HashMap;

/*
#更新HashMap
- HashMap 大小可变
- 每个K 同时只能对应一个V

- K 已经存在,对应了一个V
   - 替换现有的值
   -保留现有的值
   -合并现有的Value,和新的Value
- K 不存在
  - 添加一对 K,V

  ##  默认情况下,HashMap使用功能强大的Hash 函数,可以抵抗Dos 攻击
  - 不是可用的最快的hash 算法
  - 但具有良好的安全性
  -可以指定更快的hasher 来切换到另一个函数
  - hasher 是实现了BuilderHasher trait 
*/
fn main() {
    println!("hello world");
    example1();
    example2();
    example3();
}

fn example1() {
    let mut h = HashMap::new();
    h.insert(String::from("red"), 10);
    h.insert(String::from("red"), 20);
    println!("{:?}", h);
}

//只在没有该 K的时候插入

fn example2() {
    let mut h = HashMap::new();
    h.insert(String::from("red"), 10);
    h.entry(String::from("red")).or_insert(50);
    println!("{:?}", h);
}

// 基于现有的值来更新现值
fn example3() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
