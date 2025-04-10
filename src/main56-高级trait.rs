//高级trait
use std::fmt;
use std::ops::Add;
fn main() {
    // -------------------- 关联类型在trait 定义中指定占位符类型 --------------------
    // 比本章中的其他常见,但是在全书中本章的内容都很少见
    println!("hello world!");
    pub trait Iterator {
        //Item 是一个占位符,下面这段代码表明 不管实现者指定何种类型,next 方法都会返回一个包含此具体类型的类型值的Option ;
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct Counter {};
    //关联类型的实现者必须实现其占位符类型;
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            // println!("yyds")
            Some(1)
        }
    }
    //使用泛型的方式和这种占位符的方式有什么区别?
    //使用泛型
    pub trait Iterator1<T> {
        fn next(&mut self) -> Option<T>;
    }
    // 如果使用泛型的方式 那么可以实现Iterator<String> for Counter 以及Iterator<u32> for Counter ,或者其他任何类型,我们就可能拥有多个 Counter 的Iterator 实现,我们在使用Counter 的next 方法时 必须提供类型标注来表明希望使用Iterator 的哪个实现

    // 如果使用关联类型,则无需标注类型,因为不可以多次实现这个trait, 当使用next 的时候也不必每次指定我们需要u32类型的迭代器

    // ---------------------
    //为了能区分 调用了那个trait 的fly 方法时
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    //如果没有self 参数,但trait 和其本身都实现了同一个trait ,那么就要使用完全想定语法;
    println!("A baby dog is called a {}", Dog::baby_name());
    //------ 为Vec<T> 实现自定义Display trait
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w={}", w);
}

// -------------------- 默认泛型类型参数和运算符重载 --------------------
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
// 这个语法叫作默认类型参数 (default type parameters),RHS 是一个泛型类型参数(right hand side ) 的缩写,如果不指定RHS 的具体类型RHS 就是默认的Self 类型,
// trait Add<RHS = Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// 我们希望能将毫米值和米值相加,并让Add 的实现正确处理转换,可以为Millimeters 实现Add 并以Meters 作为RHS
// - 扩展类型而不破坏现有代码
// - 在大部分用户都不需要的特定情况自定义;
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// -------------------- 完全限定语法与消歧义 : 调用相同名称的方法 --------------------
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        // ...
    }
}
impl Wizard for Human {
    fn fly(&self) {
        //..
    }
}
impl Human {
    fn fly(&self) {
        //...
    }
}
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
// -------------------- 父trait 用于在另一个trait 中使用某trait 的功能 --------------------

// 希望某个trait 使用另外一个trait 的功能,这个所需的trait 是我们实现的trait 的父级(supertrait);
// 就比如
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct Point1 {
    x: i32,
    y: i32,
}
impl fmt::Display for Point1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
//这里如果没有实现Display trait 会报错
impl OutlinePrint for Point1 {}

// -------------------- newtype 模式用于在外部类型上实现外部trait --------------------
//在为类型实现trait 由于有孤儿规则(orphan rule) 他说明只要trait 或者类型对于当前crate 是本地的话,就可以在这个类型上实现trait 一个绕开这个的限制的方法是使用newtype 模式
// 比如为Vec<T> 实现Display
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}
