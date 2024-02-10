use state_pattern::{EncodedPost, Post};

fn main() {
    let mut post = Post::new();
    post.add_text("new text");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("new text", post.content());

    let mut encoded = EncodedPost::new();
    encoded.add_text("encoded");
    let encoded = encoded.request_review();
    let encoded = encoded.approve();
    assert_eq!("encoded", encoded.content());
}
