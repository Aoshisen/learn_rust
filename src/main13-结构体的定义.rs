/*
#定义和实例化struct
*/
fn main() {
    //实例化 struct
    let user1 = User {
        username: String::from("Ass"),
        email: String::from("aoshisen@google.com"),
        sign_in_count: 12,
        active: false,
    };
    // 如何使用
    println!(" user name {}", user1.username);
    // struct 的更新语法
    let user2 = User {
        username: String::from("Ass"),
        ..use1
    };
    // tuple struct
}
// 
struct Color(u32, u32, u32);
struct Origin(u32, u32, u32);
//空结构体
struct IsEqual;
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

// NOTE: 结构体可以持有变量的所有权,
// 在这种情况下 只要struct 的实例是有效的那么这个结构体里面的字段数据也是有效的,
// struct 里面的字段也可以存放引用,这个时候就需要用到生命周期
//    - 生命周期能保证只要struct 实例是有效的,那么struct 里面的引用也是有效的


// 使用结构体作为函数的返回值
fn build_user(username: String, email: String) {
    User {
        username: username,
        email: email,
        sign_in_count: 12,
        active: false,
    }
}
// 简便的写法
fn build_user1(username: String, email: String) {
    User {
        username,
        email,
        sign_in_count: 12,
        active: false,
    }
}

