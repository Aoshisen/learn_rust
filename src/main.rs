use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
};

/*
复习一下我们之前所提到的Result 枚举,
```rust
enum Result<T,E> {
OK(T),
Err(E),
}
在程序发生错误的时候 一般有一些是可以恢复的错误，比如我们打开文件失败了，我们可以先创建该文件，但是如果数组越界了这种错误就是不可以恢复的，就要立即报错了，
这一小节我们就会去详细的探讨一下 如何处理，可以恢复的错误以及不可以恢复的错误


```
*/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello world");
    // 如果我们指定了一个f:u32 那么如果文件打开的时候的类型不匹配就会在控制台打印为什么这个赋值是错误的，并且会输出File::open 这个函数调用之后的返回值的具体类型
    // let f: u32 = File::open("hello.txt");

    let f = File::open("hello.txt");
    // 如果成功就会返回一个可以操作的文件句柄，如果是被的话那么就返回的是一个std::io::Error 的错误实例
    //我们现在就要使用我们的match 表达式去处理返回的枚举类型里面的错误
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         //这里就会打印error ，会返回系统找不到指定的文件的error 实例，
    //         panic!("Problem opening the file {:?}", error);
    //     }
    // };

    // 同样的我们在Err 中可能会存在很多类型的错误，让我们显示的去处理她
    // 如果当前的文件没有找到，我们可以通过手动的创建同目录的文件来避免我们的错误，这样
    // let f = match f {
    //     Ok(res) => res,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file {:?}", other_error),
    //     },
    // };
    // 让我们来改进一下 上面的代码
    // 让我们使用unwrap_or_else 来改进match 表达式
    /*

    ``` rust
    let r = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
    ```
    */
    // 我们在panic! 的简写unwrap 以及expect
    // 调用unwrap 之后会发生的事情
    // 比如 类似于match 如果是成功的状态,那么就返回Ok中的值，如果失败，那么就自动的调用panic!  但是这个方法 不能自定义panic! 输出的值，如果想要控制panic！之后输出到控制台的提示信息的话，就使用expect
    /*
    ```rust
    let x = File::open("hello.txt").unwrap();
    let x = File::open("hello.txt").expect("打开文件失败1111");
    ```
    */
    // 如果在main 函数中使用？
    let f = File::open("hello.txt")?;

    //编译器会报错，因为main 函数很特殊，其值返回为一个(); 但是? 这个操作可能会返回一个Error 类型，
    // 1. 通过将返回值修改为Result<T,E>
    // 2.通过合适的方法来处理 Result<T,E>
    Ok(())
}

// 但是很多时候，我们不需要在我们代码内部去处理我们的错误，我们需要把我们代码里面的错误告知我们的函数的调用者，让调用者决定到底该如何处理这个错误，就例如我们File::open 方法打开文件的时候我们会接收到文件的打开失败的具体的Err,
// 我们可以自己处理我们的代码逻辑并且也组织好我们的错误，传递给调用者
fn read_username_from_file() -> Result<String, Error> {
    let file = File::open("hello.txt");
    let mut f = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误的简写?
// Result 值之后的? 的作用 如果Result  的值是Ok 那么 程序将继续执行，如果Result 的值是Error 那么程序就将该Error 作为整个函数的返回值返回，就好像使用了return 关键字一样
// 内部的实现是? 会把错误的值传递给form函数，收到错误之后,form 函数会把收到的错误类型转换成当前函数返回类型所指定的错误类型
fn read_username_from_file2() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

//通过链式调用进一步精简代码
fn read_username_from_file3() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}
