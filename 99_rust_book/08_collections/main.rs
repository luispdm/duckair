use unicode_segmentation::UnicodeSegmentation;

// Strings are stored as a collection of UTF-8-encoded bytes
fn main() {
    let mut s = String::from("Hell");
    let s2 = String::from(" World");
    s.push('o'); // append a single char to a string
    // three ways to append a string to another:
    // s.push_str(&s2);
    // let s3 = s + &s2; // moving ownership of s to s3
    s = format!("{}{}", s, s2); // format doesn't move ownership of s and s2
    println!("{}", s);
    
    /*
     * string indexing - s[0] is not possible in Rust
     * Words are represented in three ways in UTF-8:
     * bytes, scalar and grapheme
     */
    let hn = String::from("नमस्ते");
    println!("{}", hn);
    println!("bytes representation:");
    for b in hn.bytes() {
        println!("{}", b); // a bunch of bytes
    }
    println!("scalar representation:");
    for c in hn.chars() {
        println!("{}", c); //न, म, स, weird char, त, weird char
        
    }
    println!("grapheme representation (not implemented in std Rust):");
    for g in hn.graphemes(true) {
        println!("{}", g); //न, म, स, त
    }
}
