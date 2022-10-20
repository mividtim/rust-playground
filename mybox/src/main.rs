fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("", post.content());
    post.reject();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approvals: u8,
}

impl Post {

    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approvals: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            let (new_approvals, new_state) = s.approve(self);
            self.approvals = new_approvals;
            self.state = Some(new_state);
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
            self.approvals = 0;
        }
    }
}

trait State {
    fn content<'s>(&self, _: &'s Post) -> &'s str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, post: &Post) -> (u8, Box<dyn State>);
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>, post: &Post) -> (u8, Box<dyn State>) {
        (post.approvals, self)
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &Post) -> (u8, Box<dyn State>) {
        (post.approvals + 1, if post.approvals > 0 { Box::new(Published {}) } else { self })
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {

    fn content<'s>(&self, post: &'s Post) -> &'s str {
        &post.content
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &Post) -> (u8, Box<dyn State>) {
        (post.approvals, self)
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}