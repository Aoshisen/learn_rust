/*
#rust 中的模块(主要用于组织代码)
- 哪些细节可以暴露,哪些细节是私有的
- 作用域内哪些名称有效

- package(包) cargo 的特性: 构建,测试,共享crate
- crate(单元包) 一个模块树,可以产生一个library 或者可执行文件
- module(模块) use:可以控制代码的组织,作用域,私有路径等
- path (路径)  为struct, function,module 命名的方式

*/
// 入口文件,管理,main.rs binary crate 的 默认的文件路径就是这个,并且这个包名和项目名字相同
// 如果项目目录里面有src/lib.rs 那么该package 包含一个library crate crate 的名字也和项目名字相同
// cargo 就会把crate root 文件交给rustc 来构建library 或者binary
//一个项目可以有多个binary crate 我们可以把 binary crate 放在src/bin 里面
// 定义crate 我们可以将相关的代码放到一个作用域内,便于在 项目内进行共享
// 避免命名空间的冲突

// 定义module 来控制作用域和私有性
// 在一个crate 将代码进行分组
// 增加代码的可读性易于复用
// 控制代码的私有行,public private

// ##如何建立module
// -mod 关键字
// -可嵌套
// 可包含其他项 (struct,enum const,fn,trait) 等

// ##深入理解module
// src/main.rs  和src/lib.rs 都叫做crate roots:
// src/lib.rs 或者src/main.rs 的内容形成了一个叫crate 的模块,位与整个模块树的根部
//

fn main() {
    println!("hello world!");
}
