/*
// 切片
1. 通过实现一个程序来深入的理解切片的概念

- 1. 接受一个字符串作为参数
- 2. 返回在这个字符串中找到的第一个单词
- 3. 如果没有空格那么返回整个字符串

* */
fn main() {
    let word = String::from("hello world");
    let first_word_index = first_word(&word[..]);
    // 当前这个函数有一个问题,如果当前这个函数的返回值脱离了传入参数本身,那么该返回值就没有任何意义了,
    //当前的这个索引位置是独立于字符串单独存在的,在得到了函数的返回值后,我们就再也不能保证该值的有效性了
    // word.clear();
    // 执行了当前这个操作后,因为first_word_index 仍然是5,而这个5 在后续的代码里面就没有任何意义了,
    println!("first word {}", first_word_index);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn slice() {
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];
    /*
    ## 字符串切片
    NOTE: 字符串切片的范围索引必须发生在有效的utf-8 字符边界内
    如果尝试对一个多字节的字符中创建切片,程序会报错并退出
    */
}

// 重新之前的函数
fn first_word(a: &str) -> &str {
    let bytes = a.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &a[..i];
        }
    }
    &a[..]
}
