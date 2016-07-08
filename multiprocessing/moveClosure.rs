//! This file shows how a move-closure takes ownership from the outer function
//! Note that the moved closure gets it's own copy of `x`

use std::thread;

fn main() {
    let mut x = 1;
    println!("Original value of x is {}", x);

    let child = thread::spawn(move || { // if you comment out `move`, you will get an error
        x += 1;
        println!("thead's copy of x is {}", x);
    });

    child.join().expect("thread error");
    println!("outer x is {}", x);
}
