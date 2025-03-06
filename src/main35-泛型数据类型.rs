/*
# 使用trait 和生命周期
## 提取代码来减少重复的逻辑
*/
// fn main() {
//     // println!("hello world");
//     let number_list = vec![24, 50, 25, 100, 65];
//     let mut largest = &number_list[0];
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     println!("the largest number is {}", largest);
//     assert_eq!(*largest, 100);
// }

// fn main() {
//     // println!("hello world");
//     let number_list = vec![24, 50, 25, 100, 65];
//     let mut largest = &number_list[0];
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     println!("the largest number is {}", largest);
// 	//另一个number_list 中寻找最大值
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//     let mut largest = &number_list[0];
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     println!("The largest number is {largest}");
// }

fn main() {
    let number_list = vec![24, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is {}", result);
    //另一个number_list 中寻找最大值
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    // Rust 编译代码的时候使用单态化的过程
    // let integer = Some(5);
    // let float = Some(5.0);
    // ------------------to --------------
    //当发现Option<T> 的值有两种的时候(i32,f64) 就会把泛型的定义展开为Option_i32 和Option_f64, 然后将泛型替换为这两个具体的定义
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

// fn largest(v: &[i32]) -> &i32 {
//     let mut largest = &v[0];
//     for number in v {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// // 在函数定义重使用泛型

// fn largest_char(v: &[char]) -> &char {
//     let mut largest = &v[0];
//     for number in v {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

fn largest<T>(v: &[T]) -> &T
//需要实现PartialOrd 这个trait
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &v[0];
    for number in v {
        if number > largest {
            largest = number;
        }
    }
    largest
}

//如何在结构体里面定义泛型
struct Point<T> {
    x: T,
    y: T,
}
fn test() -> () {
    let _integer = Point { x: 10, y: 5 };
    let _float = Point { x: 1.0, y: 2.0 };
    // let fail=Point{
    // 	x:1,
    // 	y:5.0,
    // };
}

//如果我们需要使用不同的泛型参数我们就要去改写我们的point 方法里面的泛型参数
struct Point1<T, U> {
    x: T,
    y: U,
}
fn test1() -> () {
    let _integer = Point1 { x: 10, y: 5 };
    let _float = Point1 { x: 1.0, y: 2.0 };
    let _success = Point1 { x: 1, y: 5.0 };
}

// 枚举中的泛型

// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 方法定义中(在 impl 中的才叫方法,普通的未定义在impl 块中的是函数)

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
