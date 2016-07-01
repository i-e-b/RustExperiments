//! A very simple example of spawning and ending threads

use std::thread;

fn spawn_func() {
    println!("Hello threads");
}

fn main() {

    // here's a plain function being split into a thread
    let child = thread::spawn(spawn_func);

    // here's a closure being spawned
    let cchild = thread::spawn(||{ println!("Hello closure");});

    child.join().expect("child thread faulted");
    cchild.join().expect("child thread faulted");
}
