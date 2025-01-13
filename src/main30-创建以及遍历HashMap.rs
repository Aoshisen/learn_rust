/*
#HashMap HashMap<K,V>
- 键值对这种形式来 存储数据
- 使用hash 函数来决定如何在内存中存放Key 和value

-HashMap 使用的比较少,不在prelude中
- 标准库对其支持的比较少,没有内置的宏来创建hashmap
- 数据存储在heap 上
- 同构的
  - 所有的K 必须是同一种类型
  - 所有的V 必须是同一种类型
- 使用collect 来创建HashMap
## hashMap 的所有权
- 对于实现了copy trait 的类型,值会被复制到HashMap 中字符串究竟是什么
- 对于拥有所有权的值(String),值会被移动,所有权会转移给HashMap
*/
use std::collections::HashMap;
fn main() {
    println!("hello world");
    // example1();
    // example2();
    example4();
}

fn example1() {
    let mut score = HashMap::new();
    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Red"), 50);
}

fn example2() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 200];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
fn example3() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{}", s),
        None => println!("team not exist"),
    }
}

fn example4() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    for (team, score) in scores {
        println!("{},{}", team, score);
    }
}
