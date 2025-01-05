fn main() {
    //复合类型(Compound Types)有两种：元组(Tuple)和数组(Array)
    //元组(Tuple)是一个将多个其他类型的值组合进一个复合类型的主要方式
    //元组长度固定：一旦声明，它们的长度不能增长或缩小
    //元组中的每个位置都有一个类型，而且这些类型不一定要相同
    //元组的类型签名是包含在尖括号中的类型列表
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {:?}", tup);
    //如何访问元组中的元素

    //使用模式匹配解构元组
    let (x, y, z) = tup;
    println!("x is {},y is {},z is {}", x, y, z);
    //访问tuple 中的元素 使用.号和索引
    println!("tup.0 is {}", tup.0);

    //数组(Array)中的每个元素的类型必须相同
    //数组的长度是固定的，一旦声明，它们的长度不能增长或缩小
    //数组是在栈上分配的，而不是在堆上
    //数组的类型签名是包含在方括号中的类型列表
    let a = [1, 2, 3, 4, 5];
    println!("a is {:?}", a);
    // 如何声明数组的类型 [i32; 5] 表示数组中有5个元素，每个元素的类型是i32
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //如果这几个元素都是相等的 那么可以 使用 [2;5] 来快捷声明数组
    let b = [2; 5];

    //访问数组中的元素
    println!("a[0] is {}", a[0]);
    println!("b[0] is {}", b[0]);
    //
    //数组越界
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
    //数组越界 编译会通过
    //但是运行的时候会造成程序的panic
    //   --> src/main.rs:31:26
    //    |
    // 31 |     println!("b[10] is {}", b[10]);
    //    |                             ^^^^^ index out of bounds: the length is 5 but the index is 10
    //    |
    //    = note: `#[deny(unconditional_panic)]` on by default
}
