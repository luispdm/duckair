/*
 * Interior mutability: allows you to mutate data even on immutable references to that data. To allow this,
 * the pattern uses unsafe code inside data structures that bypasses borrowing rules.
 * Unsafe code is not checked at compile time but it can be checked at runtime.
 *
 * Enter: RefCell. RefCell allows mutating immutable references. The borrowing rules on RefCell
 * are checked at runtime (if you break those rules at runtime, the program will panic). Note that checking borrowing
 * rules at runtime does have a small performance hit! For this reason and for obvious safety reasons, the default
 * in Rust is checking the borrowing rules at compile time.
 * RefCell only works in single-threaded programs. There's only one owner of data with RefCell.
 * RefCell is useful when you are sure your code is following the borrowing rules, but the compiler is not able
 * to detect that. Which are those situations? The halting problem (https://youtu.be/macM_MtS_w4?si=z4lbH_3YjJIGAEye)
 * and the Messenger-Whatsapp example defined below.
 *
 * From the Rust book
 * https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#enforcing-borrowing-rules-at-runtime-with-refcellt :
 * - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners
 * - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows
 *   checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime
 * - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T>
 *   even when the RefCell<T> is immutable
 */

use std::{cell::RefCell, rc::Rc};

trait Messenger {
    fn send(&self, msg: &str);
}
/*
 * This version of "Whatsapp" raises the following error:
 * "cannot borrow `self.sent_msgs` as mutable, as it is behind a `&` reference"
 * In "send", "Whatsapp" is borrowed as an immutable reference ("&self"), thus its fields are also immutable.
 * The compiler suggests changing the signature of "send" inside "impl" to "&mut self", but this would be incompatible
 * with the send method defined in the Messenger trait.
 */
// struct Whatsapp {
//     sent_msgs: Vec<String>,
// }

// impl Messenger for Whatsapp {
//     fn send(&self, msg: &str) {
//         self.sent_msgs.push(msg.to_string());
//     }
// }

// the following version of "Whatsapp" solves the problem defined above
struct Whatsapp {
    sent_msgs: RefCell<Vec<String>>,
}
impl Messenger for Whatsapp {
    fn send(&self, msg: &str) {
        let mut a  = self.sent_msgs.borrow_mut();
        // let mut b = self.sent_msgs.borrow_mut(); // it compiles but it panicks at runtime: multiple mutable references!
        a.push(msg.to_string());

        // note that if we replace the fn body with the following:
        // self.sent_msgs.borrow_mut().push(msg.to_string());
        // self.sent_msgs.borrow_mut().push(msg.to_string());
        // the code doesn't panic because the mutable references are not assigned to any variable, thus they are
        // dropped on the next instruction!
    }
}

// combining Rc with RefCell: creating multiple owners of mutable data
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let w = Whatsapp{sent_msgs: RefCell::new(vec![])};
    w.send("a");
    w.send("b");
    println!("{:?}", w.sent_msgs.borrow());

    let v = Rc::new(RefCell::new(5));
    println!("v initial value is {}", *v.borrow());
    let a = Rc::new(Cons(Rc::clone(&v), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("{:?}\n{:?}\n{:?}", a, b, c);
    // automatic dereferencing: RefCell is extracted from Rc (v type is Rc<RefCell<i32>>)
    *v.borrow_mut() += 10; // *v.borrow_mut() extracts the i32 from RefCell
    println!("v new value is {}", *v.borrow());
    println!("{:?}\n{:?}\n{:?}", a, b, c); // the update to v will be reflected inside a, b and c
}
