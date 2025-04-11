// ## 高级类型
//1. newtype 和类型一样有用
//2. 类型别名,类似于newtype 但是稍微不同,
//3. ! 永远不回返回值的类型
//4. str 的动态大小类型
fn main() {
    // --------------------------------------- 为了类型安全使用newtype 模式  ---------------------------------------
    // 使用newtype 模式可以确保某值不被混淆, 比如Millimeters(u32) 和 Meters(u32), 其值都是 u32 类型的,如果不小心使用了不同的Meters 或者普通的u32 来调用Millimeters 上的方法,是不能编译的;
    // 使用newtype 模式可以隐藏 其内部的泛型类型, 比如 封装了一个People 类型 其 会存储People 的名字 Hash<i32,String> 但是我们在创建的时候并不希望传入显示的ID,只需像People 传递名字即可,

    // --------------------------------------- 类型别名用来创建类型同义词  ---------------------------------------
    //看下面一段 代码,可以让i32 这个类型更与语义化;
    //其实就是一个类型别名而已
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 10;
    // 简化重复长的类型

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    //     // --snip--
    // }

    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     // --snip--
    // }
    //重写为下面这种形式
    // type Thunk = Box<dyn Fn() + Send + 'static>;

    // let f: Thunk = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Thunk) {
    //     // --snip--
    // }

    // fn returns_long_type() -> Thunk {
    //     // --snip--
    // }

    // //  类型别名经常与Result<T,E>来使用
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    // fn flush(&mut self) -> Result<(), Error>;
    // //=====================变成下面这样==================
    // fn write(&mut self, buf: &[u8]) -> Result<usize>;
    // fn flush(&mut self) -> Result<()>;

    // 之所以可以这么写,是因为有类型别名,然后补齐了第二个Error 参数的默认值
    // type Result<T> = std::result::Result<T, std::io::Error>;

    // --------------------------------------- 从不返回的never type  ---------------------------------------
    // 函数bar 从不返回
    // fn bar() -> ! {
    // 	//
    // }
    //下面这个continue 里面的返回值就是!
    // let guess=String::from("hello")
    // let guess:u32=match guess.trim().parse(){
    // 	Ok(num)=>num,
    // 	Err(_)=>continue,
    // };
    // 下面这段代码不能工作因为返回的值 类型不一样,i32 和String;
    // let guess = match guess.trim().parse() {
    //     Ok(_) => 5,
    //     Err(_) => "hello",
    // }
    // 但是第一个例子却可以,是因为其返回的类型是! 他虽然是一个值,但是却用来标识没有值的值,
    // 描述 ! 行为的真正方式是 never type 可以强制转换成其他类型,
    // never type 的另外一个用途是panic
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }
    // -----------------------------loop 函数的返回值就是! -----------------------------,
    // 如果循环永远不回结束,那么其表达式的值是! ,如果使用break 结束了,那么不为真了, 如果break 指定了值,那么其值就是指定值的类型,因为! 可以强制转换成任意类型,如果没有指定值那么其返回值就是();
    print!("forever ");

    loop {
        print!("and ever ");
    }

    // ----------------------------- 动态大小类型和 Sized trait -----------------------------,
    // 因为rust 需要知道特定类型的值,然后再给其分配具体的大小,但是有一些类型其大小是不固定的只有在运行时才知道其具体的大小,这种类型就是动态大小类型 DST(或 unsized types);

    //str 本身就是一个动态大小的类型所以本书,现在都是在使用&str(str 的切片) 而不是str  &str 存储了字符串的开始位置和slice的长度;
    // 和普通的&T 不同,&T 存储了T 在内存中的单个值,但是 &str 则是两个值 地址和长度,这样我们就知道了&str 的编译时大小 它是usize的两倍,
    // 为了处理DST 动态大小类型,rust 有一个trait 来确定一个类型是否在编译时知道其大小(Sized trait)
    // fn generic<T>(t: T) {
    //     //---------
    // }
    // //实际上

    // fn generic<T:Sized>(t: T) {
    //     //---------
    // }
	// 放宽后的限制
    // fn generic<T: ?Sized>(t: &T) {
    //     //---------
    // }
}
