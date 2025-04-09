// 模式语法

fn main() {
    // ---------------------------------------- 匹配字面量 ----------------------------------------
    println!("hello world!");
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ---------------------------------------- 匹配命名变量 ----------------------------------------
    let x = Some(5);
    let y = 20;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Got y {:?}", y),
        _ => println!("default Case,x={:?}", x),
    }
    println!("at end :x ={:?},y={:?}", x, y);

    // ---------------------------------------- 匹配多个模式 ----------------------------------------
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ---------------------------------------- 通过..=语法匹配值的范围 ----------------------------------------
    // char 和 数字值是Rust 仅有的可以判断范围是否为空的类型
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    //NOTICE: char 是单个'' 而字符串是""
    let y = 'c';
    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // ---------------------------------------- 解构并分解值 (解构结构体) ----------------------------------------
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // 简写为
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // ---------------------------------------- 解构和匹配 模式中的字面量 ----------------------------------------
    let p = Point { x: 0, y: 7 };
    match p {
        //当y 为0 的时候说明在x 轴上
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        //当x 为0 的时候说明在x 轴上
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("on neither axis ({},{})", x, y),
    }

    // ---------------------------------------- 解构包含不同类型值成员的枚举 ----------------------------------------
    #[allow(unused)]
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }
    // let message = Message::ChangeColor(0, 160, 255);

    // match message {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to destructure")
    //     }
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {} and in the y direction {}", x, y)
    //     }
    //     Message::Write(text) => println!("Text message : {}", text),
    //     Message::ChangeColor(r, g, b) => {
    //         println!(
    //             "Change the color to red {}, green {}, and blue {} ",
    //             r, g, b
    //         )
    //     }
    // }
    // ---------------------------------------- 解构嵌套的结构体和枚举 ----------------------------------------
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    #[allow(unused)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    // ---------------------------------------- 解构 结构体 和 元组 ----------------------------------------
    #[allow(unused)]
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ---------------------------------------- 忽略模式中的值 ----------------------------------------
    //使用_忽略整个值
    fn foo(_: i32, y: i32) {
        println!("this code only uses the y parameter:{}", y)
    }
    foo(3, 4);
    // ---------------------------------------- 使用嵌套的_忽略部分值 ----------------------------------------
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("can't overwrite an existing customized value")
        }
        _ => {
            //如果这个值在被修改过后没有被使用 Rust 会以为这个是个潜在的错误
            setting_value = new_setting_value;
        }
    };
    println!("changed setting_value is {:?} ", setting_value);
    // 使用 多个 _ 忽略多个特定值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {} ", first, third, fifth)
        }
    }
    // ---------------------------------------- 通过在名字前以一个下划线开头来忽略未使用的变量 ----------------------------------------
    let _x = 5;
    let y = 10;
    println!("{y}");
    // NOTICE: 使用_ 和_xxx 的名称有些微妙的不同,_x 可能会导致值绑定到变量,而_ 则完全不会绑定,请看例子
    // let s = Some(String::from("Hello"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // //这里由于s 被移动了到了_s 所有权发生了变化,后续就不能使用了,所以下面的代码会出错
    // println!("{:?}", s)

    let s = Some(String::from("Hello"));
    if let Some(_) = s {
        println!("found a string");
    }
    //而使用_ 则完全不回发生移动或者绑定
    println!("{:?}", s);

    // ---------------------------------------- 用 .. 忽略剩余值 ----------------------------------------
    #[allow(unused)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }
    //取第一个和最后一个数字
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {},{}", first, last);
        }
    }
    //NOTICE: 不能使用有歧义的匹配
    //下面是一个有问题的例子
    // match numbers {
    //     (.., second, ..) => {
    //         println!("some numbers: {}", second);
    //     }
    // }

    // ---------------------------------------- 匹配守卫提供额外判断条件 ----------------------------------------
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    // 使用匹配守卫来解决模式中变量覆盖的问题-----------
    let x = Some(5);
    let y = 5;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
    // 匹配多个模式和匹配守卫
    let x = 4;
    let y = false;
    match x {
        // 所以匹配模式的优先级 看起来像是(4 | 5 | 6) if y => ...
        //而不是 4 | 5 | (6 if y) => ...
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // ---------------------------------------- 使用@ 绑定 ----------------------------------------
    enum Message1 {
        Hello { id: i32 },
    }
    let msg = Message1::Hello { id: 5 };
    match msg {
        //模式匹配id 并绑定其到id_variable 并且判断其范围是否在3-7 之间
        Message1::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        //判断id 的范围是否在10-12 之间
        Message1::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message1::Hello { id } => {
            println!("Found some other id :{}", id)
        }
    }
}
