/*
# option 枚举 ,null
//定义于标准库中, 在预导入模块中导入,
// 描述这样一种值的状态,某个值可能存在(任意类型) 或者不存在的状态
null 本身就是一个值,表示没有值
Option<T>
如果 想使用Option<T> 那么就要 手动转换T
*/
fn main() {
    let some_number = Some(5); //有效的值
    let some_string = Some("A string"); //有效的值
    let null: Option<i32> = None;
    let x: i8 = 5;
    let sum = x + some_number;
    //error[E0277]: cannot add `Option<i8>` to `i8`
    //   --> src/main.rs:13:17
    //    |
    // 13 |     let sum = x + some_number;
    //    |                 ^ no implementation for `i8 + Option<i8>`
}
