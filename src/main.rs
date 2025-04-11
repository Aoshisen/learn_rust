//## 宏
//1. 自定义#[derive] 在结构体和枚举上指定derive属性添加的代码
//2. 类属性,可用于任意项 的自定义属性
//3. 类函数宏,看起来像函数不过作用域作为函数传递的token;
//宏展开,让你生成比你手写出的更多的代码
// 宏就是生成代码的代码,所以比普通函数更难阅读理解和维护
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!("hello world!");

    // ----------------------------------------------- 最常用的声明宏 --------------------------------
    // 声明宏允许我们编写一些类似与Rust match 表达式的代码,来进行匹配,而传递进来的值是rust 的源代码;
    // 例如 vec 宏

    //使用 macro_export 表明只要将宏的crate 引入作用域,宏就是可用的,如果没有标注,那么宏不能被引入作用域;
    // #[macro_export]
    // macro_rules! vec {
    // 首先一对括号包含了所有模式, 然后$() 这相当于是一个模式,需要被提取出来的部分,然后后面的逗号 说明是可有可无的,然后后面的* 说明该模式可以匹配0个或者多个* 之前的任意模式 expr 赋值给了$x 表示匹配到的任何表达式都将其记做$x
    // ( $( $x:expr ),* ) => {
    //     {
    //         let mut temp_vec = Vec::new();
    //         $(
    //             temp_vec.push($x);
    //         )*
    //         temp_vec
    //     }
    // };
    // }

    // ----------------------------------------------- 用属性生成代码的过程宏 --------------------------------
    // 有三种类型的过程宏(自定义派生,derive, 类属性, 和类函数)
    // 使用过程宏必须驻留在他们自己的具有特殊crate 类型的crate 中,
    // 一个使用过程宏的例子
    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {}
    //我们希望能像这样使用宏
    Pancakes::hello_macro();
}

// 我们可能在src/lib.rs 下面(在上层目录下创建 cargo new hello_marco --lib 创建库)
// use hello_marco::HelloMarco
pub trait HelloMacro {
    fn hello_macro();
}
//然后在本main 函数中引入和使用宏
impl HelloMacro for Pancakes {
    fn hello_macro() {}
}
//但是我们需要 使用hello_marco 的类型编写实现的代码块,并且简化这个操作,所以我们再在hello_marco 项目中使用  cargo new hello_marco_derive --lib 创建一个derive 的crate;
//改变其cargo.toml
// [lib]
// proc-macro = true

// [dependencies]
// syn = "1.0"
// quote = "1.0"

//syn 可以将字符串中的rust 代码解析称为一个可以操作的数据结构, 而quote 则是把解析的数据结构转换成rust 代码; 就相当于是语言的解析器和生成器
//hello_marco_derive 的实现
// extern crate proc_macro;

// use crate::proc_macro::TokenStream;
// use quote::quote;
// use syn;

// #[proc_macro_derive(HelloMacro)]
// pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//     // 将 Rust 代码解析为语法树以便进行操作
//     let ast = syn::parse(input).unwrap();

//     // 构建 trait 实现
//     impl_hello_macro(&ast)
// }

// fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
// //为#name 实现HelloMarco 
//     let gen = quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}", stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }

// --------------------------------------------- 使用类属性宏 -----------------------------------------------------------------
// #[route(GET, "/")]
// fn index() {

//类属性宏的实现函数签名看起来像这样
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {


// --------------------------------------------- 使用类函数宏 -----------------------------------------------------------------
//看起来像这样使用
// let sql = sql!(SELECT * FROM posts WHERE id=1);

//类函数宏的实现签名看起来像这样
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {