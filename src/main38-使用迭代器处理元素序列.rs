//# 使用迭代器和for 循环执行一些代码
// rust 的迭代器是惰性的,知道迭代器被使用的时候才能产生效果.
// 迭代器会被代码消费,next 每调用一次都会消费迭代器中的一个项,使用for 循环时无需使v1_iter 可变是因为for 循环会获取v1_iter 的所有权,并在后台使v1_iter 可变;
// 使用iter() 生成不可变引用的迭代器 &T;
// 使用 into_iter() 生成一个具有所有权的迭代器 T;
// 使用iter_mut() 生成一个可变引用的迭代器 &mut T (这个需要原始数据是 mut 的);
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    //这里如果对于v1 进行下面的操作也是可以成功的,唯一的区别就是所有权,for in v1 val是持有所有权的i32 类型,而for in v1_iter 则是&i32 类型的引用类型不持有所有权;
    let s: i32 = v1_iter.sum();
    println!("s:{}", s);
    //如果一个迭代器被消费了,那么后续就不能使用了
    // for val in v1_iter {
    //     println!("Got : {}", val)
    // }
    // 产生新的迭代器
    let v2_iter = v1.iter().map(|x| x + 1);
    for val in v2_iter {
        println!("this is v2 value{}", val)
    }
    //使用迭代器产生新的vec
    let v3: Vec<_> = v1.iter().map(|x| x + 2).collect();
    println!("v3 is {:?}", v3);
    let v4 = Counter::new();
    let v4_sum: u32 = v4.sum();
    println!("v4_sum is {}", v4_sum);
}

//Iterator trait 和next 方法
// pub trait Iterator {
//     type Item; //(实现这个trait 的时候还需要同时定义一个Item 类型 关联类型);
//     fn next(&mut self) -> Option<Self::Item>;
// }

// 消费迭代器的方法, 消费迭代器的方法Iterator trait 有一系列由标准库提供的默认实现方法,这些调用了 next 的方法称之为 消费适配器,例如

// #[test]
// fn iterator_sum() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     let total: i32 = v1_iter.sum();

//     assert_eq!(total, 6);
// 在调用完成迭代器之后就不允许使用迭代器了,因为调用sum 会获取迭代器的所有权
// }

//为自定义的struct 实现 Iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
