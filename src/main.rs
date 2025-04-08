// 面向对象设计模式 实现一个功能
// 1. 博文从空白的草案(draft) 开始
// 2. 一旦草案完成,请求审批博文
// 3. 一旦博文通过审批,就会被发表
// 4. 只有被发表的博文的内容会被打印,这样就不会以外打印出没有被审批的博文的文本
fn main() {
    let mut post = Post::new();
    post.add_text("some text I pushed");
    post.request_review();
    post.approve();
    println!("post is {}", post.content());
}

pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}
impl Post {
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn new() -> Post {
        Post {
            content: String::from(""),
            state: Some(Box::new(Draft {})),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

pub trait State {
    //尝试为其添加默认实现,返回self,但是这会违反对象安全性,因为trait 不知道self 具体是什么
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}
pub struct Draft {}
impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

pub struct PendingReview {}
impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
pub struct Published {}

impl State for Published {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        &_post.content
    }
}

// 使用rust 推荐的模式来进行 编码

// fn main() {
// 蕴含着 哲学,最先通过Post 的关联函数创建了一个Draft 类型的Post
//     let mut post = Post::new();

// 然后Draft 类型的Post 内部有 content 字段保存 传入的文章
//     post.add_text("I ate a salad for lunch today");

//  状态改变成PendingReviewPost 并传入content, 
//     let post = post.request_review();

//PendingReviewPost 只有approve 方法 审批通过,返回content 被赋值的Post;
//     let post = post.approve();
//     println!("content is {}", post.content());

//     // assert_eq!("I ate a salad for lunch today", post.content());
// }
// pub struct Post {
//     content: String,
// }
// pub struct DraftPost {
//     content: String,
// }
// impl Post {
//     pub fn new() -> DraftPost {
//         DraftPost {
//             content: String::new(),
//         }
//     }
//     pub fn content(&self) -> &str {
//         &self.content
//     }
// }
// impl DraftPost {
//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }
//     pub fn request_review(self) -> PendingReviewPost {
//         PendingReviewPost {
//             content: self.content,
//         }
//     }
// }
// pub struct PendingReviewPost {
//     content: String,
// }

// impl PendingReviewPost {
//     pub fn approve(self) -> Post {
//         Post {
//             content: self.content,
//         }
//     }
// }
