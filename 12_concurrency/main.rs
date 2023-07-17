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
 */
fn main() {
    //threads basics
    let var = 412;
    let jh = thread::spawn(move || { // "move" because I need the closure to take ownership of var
        var * 2
    });
    // join ~= "waitGroup.wait()" in Go, plus it captures the return value coming from the thread
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
