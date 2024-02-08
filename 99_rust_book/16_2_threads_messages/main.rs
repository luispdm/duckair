use std::sync::mpsc; // mpsc = multi-producer, single-consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // a sender for each thread (multi-producer), as each thread takes ownership of that sender
    let tx3 = tx.clone();

    thread::spawn(move || {
        let msg = String::from("hi"); // the data type determines the generic of tx and rx, i.e. Sender<String>, Receiver<String>
        tx.send(msg).unwrap(); // you wanna handle things gracefully in a real app
        // println!("msg: {}", msg); // compilation error: send has taken ownership of msg. this prevents modifying msg after it has been sent
    });

    // to use the sender, tx must be moved inside the spawned thread as it is not safe to share it with the main thread

    let received = rx.recv().unwrap(); // recv blocks the main thread, waiting for a result from the sender
    println!("Got: {}", received);
    // try_recv doesn't block the main thread and returns a result immediately: useful when checking for new messages in a queue
    // loop {
    //     match rx.try_recv() {
    //         Ok(item) => {
    //             println!("Got: {}", item);
    //             break;
    //         },
    //         _ => println!("not ready, retrying...") // you wanna handle "Err(err)" properly in a real app
    //     }
    // }

    thread::spawn(move || {
        let msgs = vec![String::from("first"), String::from("second")];
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let msgs = vec![String::from("third"), String::from("fourth")];
        for msg in msgs {
            tx3.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for r in rx { // rx can be used as an iterator: when the channel closes, the iteration ends
        println!("Got: {r}"); // the output order is non-deterministic
    }
}
