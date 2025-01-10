/*
# 使用super ,pub struct enum
- 使用super 关键字访问父级
## pub struct
 使用pub 关键字把结构体声明为公共的
 struct 是公共的了还不行,
 struct 里面的字段默认也是私有的,
 所以要把struct 里面的字段变成公共的,就需要在每个字段前面加上pub 关键字
 ## pub enum
- enum 定义成了公共的之后枚举里面的所有字段都会变成公共的了
*/
fn main() {
    println!("hello world")
}

fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }
    fn cook_order() {}
}
