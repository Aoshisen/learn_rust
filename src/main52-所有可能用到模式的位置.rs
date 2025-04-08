// 模式和匹配
// 1. 字面量
// 2. 解构的数组,枚举,结构体或者元组
// 3. 变量
// 4. 通配符
// 5. 占位符
// 所有可能会用到模式匹配的位置
fn main() {
    println!("hello world!");
    //-------------- match 分支----------------
    // match VALUE {
    // 	PATTERN=>EXPRESSION
    // }

    //-------------- if let 条件表达式----------------
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        //如果有最喜欢的颜色那么就使用最喜欢的颜色
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        //如果是星期二那么就是绿色
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        //如果 年龄大于30 用紫色
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            //年龄小于等于三十 用橙色
            println!("Using orange as the background color");
        }
    } else {
        //其他情况使用蓝色
        println!("Using blue as the background color");
    }

    //-------------- while let  条件循环 ----------------
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //-------------- for 循环 (获取模式) ----------------
    let v = vec!['a', 'b', 'c'];

    //使用enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引,他们位于同一个元组中 第一个enumerate 调用会产生 元组(0,'a'),当这个匹配模式(index,value),index 将是0,value 将是'a'.
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //-------------- let 语句 ----------------
    // x 是一个表示 "将匹配到的值绑定到x 变量" 的模式, 因为x 是一整个模式,所以这个模式实际上等于 "将 任何值绑定到变量x"
    let x = 5;
    //使用模式解构元组并一次创建三个变量
    let (a, b, c) = (1, 2, 3);
    //如果模式中的元素不匹配元组中元素的数量,则整个类型不匹配,并会得到一个编译时错误. (如果希望忽略元组中的一个或者多个值,可以使用_ 或者.. )
    //let (c, d) = (1, 2, 3);
    // let PATTERN=EXPRESSION
}

//-------------- 函数参数 ----------------

// x其实也是一个模式,类似与let x=5;
pub fn foo(x: i32) {
    // 代码
}

//这里也是一个模式
pub fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

pub fn test() {
    let point = (3, 5);
    print_coordinates(&point);
}
