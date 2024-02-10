pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // as_ref returns Option<&Box<dyn State>>, but because of deref coercion,
        // we are able to call "content" directly on State
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // as Rust doesn't allow unpopulated struct fields and I want to move
        // state out of post, I had to declare state as an Option. In this way
        // I am able to call "take", which moves the value out of state, leaving
        // "None" in its place
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // content needs to specify a lifetime parameter as it has two borrowed parameters
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {}) // requesting a review on a draft state -> invalidating the whole state
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // no need to do anything as the post is already in the pending review state
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

/*
 * All of the previous work has two downsides:
 * 1. some states are coupled to each other: if I were to implement a new "Scheduled" state,
 * I'd have to change the code in "PendingReview" to transition to "Scheduled", before going to
 * "Published"
 * 2. code duplication: request_review and approve in Post are very similar. To get rid of that
 * duplication I'd have to use macros in Rust
 *
 * Let's see another approach to implement the State Design Pattern
*/

// let's encode the State as different types, so I don't have to implement a State trait
pub struct EncodedPost {
    content: String,
}

pub struct EncodedDraft {
    content: String,
}

impl EncodedPost {
    // the only way to create a draft is to call new on EncodedPost
    pub fn new() -> EncodedDraft {
        EncodedDraft {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

// with this implementation, draft doesn't need to have a content method
impl EncodedDraft {
    // the only way I can add text is if my post is in draft
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }
    // I cannot approve a draft, nor print its content, I can only request a review
    // this method takes ownership of self, so it invalidates the previous state
    pub fn request_review(self) -> EncodedPendingReview {
        EncodedPendingReview {
            content: self.content,
        }
    }
}

pub struct EncodedPendingReview {
    content: String,
}

impl EncodedPendingReview {
    // the only way I can approve a post is when it's in pending review
    pub fn approve(self) -> EncodedPost {
        EncodedPost {
            content: self.content,
        }
    }
}

/*
 * This second implementation doesn't quite follow the typical OOP state design pattern, but
 * invalid states are still impossible to obtain thanks to the type system and type checking.
 *
 * Using OOP design patterns won't always be the best approach in Rust because of features
 * such as ownership, which typical OOP languages don't have. We can still leverage the features
 * of Rust to implement some patterns in a different way
*/
