/*
# use 关键字的使用
## 使用pub use 重新导出名称
## 使用外部包 package
1. 在cargo.toml 添加依赖包
2. https://crates.io 下载包
3. 在代码内部使用包
4. 标准库的引用, 不需要安装了,但是需要使用显示引用
## 使用嵌套路径 清理大量的use 语句
- 如果使用同一个包或包模块下的多个条目
- 可使用 嵌套路径在同一行内将上诉条目直接进行引入
> 路径相同的部分::{路径差异的部分,路径差异的另一部分}
- 如果没有差异的部分,使用self 代替 use std::io  use std::io::Write; => use std::io::{self,Write}

## 使用通配符把 所有的公共条目引入进当前
- 使用通配符 * 可以把路径中所有的公共条目都引入到当前作用域
- 谨慎使用这个特性

使用通配符的场景
- 测试.将所有被测试的代码引入到tests 模块
- 有时被用于 prelude 模块,预导入模块

*/
// use  rand::Rng;
fn main() {
    println!("hello world");
}
