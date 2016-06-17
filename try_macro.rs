use std::fmt;

// Structure containing only a `Vec`
struct List(Vec<i32>);

fn main() {
    let v = List(vec![1, 2, 3]); // define a list...
    println!("{}", v);           // ...and print it, just to trigger the display trait
}

// display trait for list
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self; // dereference

        // Implementing fmt::Display for a structure where the elements must each be
        // handled sequentially is tricky. The problem is that each write! generates
        // a fmt::Result. Proper handling of this requires dealing with all the results.
        // Rust provides the try! macro for exactly this purpose.

        try!(write!(f, "[")); // do side effect of write, and return only on error

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}", v));
        }

        // Note: you MUST NOT have a semi-colon at the end of this line, otherwise you get
        // non-exhaustive return error [E0269]
        write!(f, "]") // final write WITHOUT `try` so we return the `fmt::Result` for this function.
    }
}

