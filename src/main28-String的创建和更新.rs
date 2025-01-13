/*
#String
> rust 开发者经常会被字符串困扰的原因
- rust 更 倾向于暴露可能存在的错误
- 字符串数据结构复杂
- 采用UTF-8 的编码

## 在rust 中字符串究竟是什么
- byte 的集合
- 提供了一系列方法将byte 解析为文本

## 在rust 核心语言层面的 字符串
- rust 只有一个字符串类型 &str 字符串切片
- 字符串切片 对存粗在其他地方,utf-8 形式的字符串的引用
- 字符串字面值,存储在二进制文件中,也就是字符串切片

## String
- 来自标准库 而不是核心语言
- 是一种可增长的,可修改的,存储在堆内存上的,可拥有其所有权的类型
- UTF-8
- 标准库提供的其他的字符串类型: OsString,OsStr,CsString,CsStr

*/

fn main() {
    //创建一个空的字符串类型
    let s = String::new();
    //使用to_string 方法来创建带有默认值的字符串(可用于实现了Display trait 的类型,包括字符串字面值);
    let data = "initial contents";
    let s1 = data.to_string();
    // 使用String::from
    let _s2 = String::from("initial value");
    //
    println!("{},{}", s, s1);
    // update();
    add();
}

/*
## 更新String
 */

fn update() {
    let mut s = String::from("foo");
    // 附加字符串切片
    s.push_str("bar");
    // 附件单个字符
    s.push('c');
    println!("{}", s);
}

// 拼接字符串

fn add() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    let s4 = format!("{}-{}-{}", s1, s2);
    println!("{}", s4);
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);
}
