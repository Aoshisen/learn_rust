fn main() {
    //字符串"10"转换为数字 但是这里没有指定数字类型，所以编译器无法推断出具体的数字类型,所以我们需要指定数字类型
    //   let num: u32 = "10".parse().expect("Not a number!");
    //   |         ^^^ help: if this is intentional, prefix it with an underscore: `_num`
    // let num = "10".parse().expect("Not a number!");
    let num: u32 = "10".parse().expect("Not a number!");
    println!("num is {}", num);
    // rust 的标量类型(Scalar Types)有四种: 整型、浮点型、布尔型和字符型
    // 1. 整数类型 u32(0-2^32-1),i32(-2^31-2^31-1),u64,i64,u128,i128,isize,usize
	// 2. 浮点类型 f32,f64
	// 整数类型 默认是i32
	//浮点类型默认是f64
	let x = 2.0; // f64
	let x=32;
	//字符类型 char 
	// char 类型是单引号，而不是双引号
	// char 类型是 Unicode 标量值，这意味着它可以比 ASCII 表示更多的内容
	// char 类型的大小为四个字节
	//范围是 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 之间的任意 Unicode 字符
	// unicode 没有字符的概念，只有标量值
	let c:char='z';

}
