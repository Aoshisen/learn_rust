use std::slice;
//static 必须声明类型
static HELLO_WORLD: &str = "hello world";
static mut COUNTER: u32 = 0;
// 高级特性- 不安全rust
// 不安全rust 提供额外的超能力;
// 不安全rust 之所以存在 是因为静态分析本质上是保守的,当编译器确定一段代码是否支持某个保证时,它最好拒绝一些有效的程序,而不是接受一些无效的程序,这也是Rust 安全的特性的保证.
// 这个时候我们就必须要使用不安全的rust 来告诉Rust 编译器,我知道我在干什么,但是这样做的后果,比如解引用空指针,导致不安全的内存使用.
// Rust 不安全的另一个方面是: 底层计算机硬件固有的不安全性,如果rust 不允许进行不安全的操作,那么有些任务根本完成不了.

// 在什么时候使用不安全rust 带来的超能力
// 1. 解引用裸指针
// 2. 调用不安全的函数或方法
// 3. 访问或修改可变静态变量
// 4. 实现不安全trait
// 5. 访问union 的字段
fn main() {
    println!("hello world!");
    // ---------------------- 解引用原始指针 ----------------------
    let mut num = 5;
    // 创建不可变的原始指针
    let r1 = &num as *const i32;
    // 创建可变的原始指针
    let r2 = &mut num as *mut i32;

    //
    let address = 0x12345usize;
    let r3 = address as *const i32;
    // rust 可以在安全代码中创建裸指针,但是不能 解引用裸指针 和读取其指向的数据
    unsafe {
        //解引用的时候才是不安全的
        println!("r1 is :{}", *r1);
        println!("r2 is :{}", *r2);
        //这个就是不安全的
        // println!("r3 is :{}", *r3);
    }
    // ---------------------- 调用不安全函数或方法 ----------------------
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // ---------------------- 创建不安全代码的安全抽象 ----------------------
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // 我们需要借用一个vec 的两个不同的部分: 编译器只知道我们借用了同一个slice 两次, 实际上借用slice 的不同部分是可以的,因为两个结果不会有重叠,但是rust 没有智能到这种地步.

    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();
    //     assert!(mid <= len);
    //     //这里的代码必须要通过unsafe 实现
    //     (&mut slice[..mid], &mut slice[mid..])
    // }
    // 使用unsafe 块,裸指针,和一些不安全函数调用来实现
    #[allow(unused)]
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        assert!(mid < len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    //直接使用unsafe  slice::from_raw_parts_mut 可能导致代码崩溃,
    // let address=0x12345usize;
    // let r=address as *mut i32;

    // let slice:=unsafe {
    // 	slice::from_raw_parts_mut(r,1000 )
    // }

    // ---------------------- 使用extern函数调用外部代码 ----------------------
    //rust 代码可能与其他语言编写的代码进行交互,extern 用于创建和使用外部函数接口 (Foreign Function Interface,FFI),外部函数接口是一个编程语言用以定义函数的方式,其允许不同编程语言调用 这些函数;
    //集成C 标准库中的abs 函数
    extern "C" {
        //extern 中的代码总是不安全的,所以无需手动标注,但是调用这个函数的时候
        fn abs(input: i32) -> i32;
    }

    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }
    // C 部分定义了外部函数所使用的应用 二进制接口(application binary interface ,ABI) --ABI 定义了如何在汇编语言层面调用此函数.

    //使用extern 来定义一个允许从其他语言调用Rust 函数的接口. 在fn 关键字前使用extern 指定需要用到的ABI, 还需要#[no_mangle] 来告诉编译器不要压碎该变量名(就是改变函数名 增加编译过程的额外信息,但是会使名称难以阅读),
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        print!("Just called a Rust function from C!");
    }
    // ---------------------- 访问或者修改可变静态变量 ----------------------
    //定义和使用一个不可变静态变量;
    println!("name is {}", HELLO_WORLD);
    //读取和修改一个可变静态变量是不安全的; 如果多线程访问并修改COUNTER 可能会造成数据竞争;
    // 如果需要在多线程里面访问数据,那么优先使用并发技术和线程安全智能指针;
    add_to_count(3);
    unsafe { println!("COUNTER : {}", COUNTER) }

    // ---------------------- 实现不安全的trait ----------------------
    // 如果想为一些不是Sync 或者是Send 的类型,比如裸指针,并将其标记为Send 或Sync ,就必须要使用unsafe,
    unsafe trait Foo {}
    unsafe impl Foo for i32 {}
    // ---------------------- 访问联合体中的字段 ----------------------
	//union 和struct ,但是在实例中智能使用一个声明的字段,这个主要和c代码中的联合体交互,访问联合体中的字段是不安全的.

	
}

fn add_to_count(inc: u32) {
    unsafe { COUNTER += inc }
}
