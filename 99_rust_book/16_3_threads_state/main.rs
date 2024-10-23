use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let m = Mutex::new(5);
    {
        // lock blocks the current thread until that lock can be acquired
        let mut num = m.lock().unwrap(); // lock returns a result bc other threads might have called lock first
                                         // num is a MutexGuard smart pointer whose deref trait points to the inner data of the mutex
        *num = 6; // when num is dropped (i.e. the drop trait of the MutexGuard is called), the lock is automatically released
    }
    // even though Rust has a lot of safeguards, you can still create deadlocks!
    // for example: if i didn't lock the mutex in a new scope before, i'd be deadlocking here
    println!("mutex's data now is: {:?}", m.lock().unwrap()); // 6

    // Arc is the atomic reference-counting smart pointer i.e. it is a thread-safe, reference-counting smart pointer
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // i cannot simply use counter here as its ownership will be moved into the thread of the first iteration
            let mut num = c.lock().unwrap();
            *num += 1;
            /*
             * Even though c is immutable, I was able to get a mutable reference to the value that it points to. How?
             * Mutex uses interior mutability. In the same way that RefCell allows you to mutate a value inside Rc,
             * Mutex allows you to mutate a value inside Arc
             */
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("counter incremented to: {}", counter.lock().unwrap()); // 10
}
