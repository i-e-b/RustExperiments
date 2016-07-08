//! Showing use of both `Arc` (atomic reference counted container)
//! and the `Mutex` data wrapper. Shows use of the 'lock data not code' priciple.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main(){
    let shared_data = Arc::new(Mutex::new( vec![0,0,0] ));
    let mut threads = vec![];

    for i in 0..3 { // over locking would give 9 second exe, correct is 3 seconds (see the delays below)
        let data = shared_data.clone();

        // And here is using a `drop` to release it explicitly:
        threads.push(thread::spawn(move ||{
            for _ in 0..15 { // entire thread should be around 3 seconds
                thread::sleep(Duration::from_millis(50)); // pretend to work
                let mut data = data.lock().unwrap();
                data[i] += 1;
                drop(data); // you can explicitly release the lock like this, but the end of the
                            // `for` loop does it implicitly if you don't
                thread::sleep(Duration::from_millis(150)); // pretend to work
            }
        }));
    }

    for t in threads {
        t.join().expect("thread error");
    }

    // We also have to lock to read the data here
    print!("Result:");
    let data = shared_data.lock().unwrap();
    for r in data.iter() { // we have the `.iter()` here because `data` is a `MutexGuard` around the `Vec`
        print!("{}; ",r);
    }
    println!("");
}

