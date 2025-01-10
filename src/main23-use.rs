/*
# 使用use 关键字 相当于是 文件系统中的symbolic link
使用use 关键字可以把其他模块的东西导入到当前作用域
> use 的习惯用法
函数 将函数的父级模块引入作用域(指定到父级)
struct enum 其他 : 指定完整路径(指定到本身)
如果两个 enum 或者struct 同名了,需要指定到父级
或者 使用 as 关键字 在本地起一个别名
*/
use learn_rust::front_of_house;
fn main() {
    front_of_house::hosting::add_to_waitlist();
    println!(" hello world");
}
