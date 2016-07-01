//! Sending data between theads

use std::sync::mpsc; // `mpsc` is "Multi-producer, single consumer"
use std::thread;

static NTHREADS: i32 = 3;

fn main () {
    // create a new channel, with a `Sender<T>` (tx)
    // and a `Receiver<T>` (rx)
    let (tx, rx) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move ||{ // need the `move` here, as the tx can't be shared with the parent thread
            thread_tx.send(id).expect("The other end hung up?");
            println!("thread {} is done", id);
        });
    }

    // collect the messages
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv().unwrap());  // let error results propagate up, otherwise get at the sent data
    }

    println!("{:?}", ids);
}
