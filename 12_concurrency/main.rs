use std::thread;

/*
 * Native threads and green threads
 *
 * Native threads 1:1 OS threads
 * Green threads m:n OS threads
 *
 * The system doesn't know anything about green threads, they are
 * entirely managed by my code
 *
 * Natively, Rust only supports native threads.
 * However, there are external libraries out there providing support
 * for green threads
 */
fn main() {
    let var = 412;
    let jh = thread::spawn(move || {
        var * 2
    });
    // join ~= "waitGroup.wait()" in Go, plus it captures the return value coming from the thread
    let res = jh.join();
    match res {
        Ok(val) => {println!("{} * 2 = {}", var, val)},
        Err(_) => {println!("Thread panicked!")}
    }
}
