//### 高级函数和闭包
fn main() {
    //1111
    println!("hello world!");
    // -------------------------------------------------- 函数指针 --------------------------------------------------
    // 我们可以向函数传递一个闭包,也可以向函数传递一个常规函数,传递的函数被称为函数,指定的参数为函数指针的闭包类似闭包
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
    //使用闭包的例子
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // 使用函数指针的例子

    let list_of_numbers = vec![1, 2, 3];
    //这里必须使用完全限定语法
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // 暴露元组结构体和元组结构体枚举成员的实现细节,
    enum Status {
        Value(u32),
        Stop,
    }

    //批量创建Status
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // 有些人喜欢闭包有些人喜欢函数指针,但是这两种写法就只是编程风格的差异,其最终都会产生相同的代码;

    // -------------------------------------------------- 返回闭包 --------------------------------------------------
    //这段代码是不能编译的,因为编译器不知道需要多少空间来存储闭包;,所以我们可以使用trait 对象来存储这个闭包
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
