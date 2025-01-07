// 循环
fn main() {
    // println!("Hello world");
    // let mut counter = 0;
    // loop {
    //     println!("again!");
    //     counter += 1;
    //     // println!("{}", counter);
    //     if (counter == 10) {
    //         break;
    //     }
    // }
    // loop 值 会一直执行,知道遇到break;
    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    //使用for 循环遍历一个数组
    // 1. 为什么不使用loop 和while 循环来遍历数组, 效率低下,且容易写错
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // 2. 使用for 循环来遍历数组
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // 小技巧, 由于for 循环的高效且安全的特性,可以使用for 循环来实现while 循环的相关功能
    // 3. 使用for 循环来实现while 循环
	//包含1 不包含4
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
