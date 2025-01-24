fn main() {
    // println!("hello world");
    // 让程序崩溃
    // panic!("crash and burn");
    //调试信息会让我们使用backtrace 来寻找代码里面出错 的地方，然后我们可以来试试 如果我们的代码不直接panic! 而是错误的调用别的库中的代码
    let v = vec![1, 2, 3];
    v[99];
}

// 在windows 系统中的powershell 中设置环境变量并运行cargo run 得到具体的报错信息
// ```powershell
// $env:RUST_BACKTRACE=1
// cargo run
// ``
/*
注意： 为了获取这些标识，我们必须启用debug 标识，当不使用 --release 参数运行cargo build 或者是cargo run 的时候debug 标识默认启动，
*/