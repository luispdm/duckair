use std::{thread, sync::mpsc, sync::mpsc::{Receiver, Sender}};

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
 *
 * From the Rust book:
 * Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems
 * were two separate challenges to be solved with different methods. Over time, the team discovered
 * that the ownership and type systems are a powerful set of tools to help manage memory safety
 * and concurrency problems! By leveraging ownership and type checking, many concurrency errors
 * are compile-time errors in Rust rather than runtime errors. Therefore, rather than making you spend
 * lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs,
 * incorrect code will refuse to compile and present an error explaining the problem. As a result,
 * you can fix your code while you’re working on it rather than potentially after it has been shipped to production.
 * We’ve nicknamed this aspect of Rust FEARLESS CONCURRENCY. Fearless concurrency allows you to write code that
 * is free of subtle bugs and is easy to refactor without introducing new bugs.
 */
fn main() {
    //threads basics
    let var = 412;
    let jh = thread::spawn(move || { // "move" because I need the closure to take ownership of var
        var * 2
    });
    // join ~= "waitGroup.Wait()" in Go, plus it captures the return value coming from the thread
    let res = jh.join();
    match res {
        Ok(val) => {println!("{} * 2 = {}", var, val)},
        Err(_) => {println!("Thread panicked!")}
    }

    // threads communication - mpsc = multi-producer, single-consumer
    let (alice_tx, alice_rx): (Sender<&str>, Receiver<&str>) = mpsc::channel();
    let (bob_tx, bob_rx): (Sender<&str>, Receiver<&str>) = mpsc::channel();
    let a_h = thread::spawn(move || {    
        alice_chat(bob_tx, alice_rx)
    });
    let b_h = thread::spawn(move || {
        bob_chat(alice_tx, bob_rx)
    });
    if a_h.join().is_ok() {}
    if b_h.join().is_ok() {}
}

fn alice_chat(bob_tx:Sender<&str>, alice_rx:Receiver<&str>) {
    match alice_rx.recv() {
        Ok(res) => {
            println!("Incoming message for Alice: {}", res);
        }
        Err(err) => {
            println!("Cannot fetch message for Alice: {}", err);
        }
    }
    let _send_msg = bob_tx.send("Hey Bob! How you doin'?!");
}

fn bob_chat(alice_tx:Sender<&str>, bob_rx:Receiver<&str>) {
    let _send_msg = alice_tx.send("Hi Alice! It's a me, Bob");
    match bob_rx.recv() {
        Ok(res) => {
            println!("Incoming message for Bob: {}", res);
        }
        Err(err) => {
            println!("Cannot fetch message for Bob: {}", err);
        }
    }
}
