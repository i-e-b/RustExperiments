// Run command line `rustdoc` on this file to get the output, then view the
//     ./doc/rusdoc_example/index.html
// file in a web-browser.
// If you add the `--test` flag, the code inside default fences will be tested,
// although I haven't got it to work correctly yet.
//
// Note this odd structure -- it is a comment for the block that encloses it
// (in this case, the whole module)
//! A file that demontrates some RustDoc features
//! Have a look at the source for notes on how the comments
//! and documentation system works.
//! [Rustdoc documentation](http://doc.rust-lang.org/book/documentation.html#documenting-modules).


/// A point in 4-space
pub struct PointM {
    x : f64,
    y : f64,
    z : f64,
    t : f64
}

/// A person structure and implementation
pub struct Person {

    /// The name
    name: String,

    /// The place in the universe
    place: PointM
}

impl Person {

    /// Returns a new person and gives them a name
    ///
    /// # Arguments
    /// * `name` - a string slice that holds the name of the person
    ///
    /// # Example
    /// The Rust doc format is essentially *Markdown*. Yay -- **another** flavour!
    /// There seems to be a problem with `test`ing code blocks in objects.
    /// Nothing resolves properly. To take a Rust block out of the test set,
    /// put its type as `ignore`.
    ///
    /// ```ignore
    /// // By default, code blocks are Rust syntax hilited
    /// let person = Person::new("name");
    /// ```
    ///
    /// If you need to have monospace text that is not rust, you can add a
    /// language specifier -- here we are using `text`
    ///
    /// ```text
    ///   +-----+
    ///   | YO! |
    ///   +-----+
    /// ```
    ///
    /// ## Note
    /// If your code block/fences don't have blank doc-comment lines above, they
    /// won't display properly.
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
            place: PointM {x:0.0, y:0.0, z:0.0, t:0.0 } // each person starts as their won centre
        }
    }

    pub fn greet(& self) {
        println!("Hello, I am {}.", self.name);
    }
}

/// The main function. Let's test the `Person` class
///
/// ```ignore
/// let person = Person::new("Tim");
/// ```
///
/// Nope, that doesn't work either.
/// Keep an eye on [Issue 23314](https://github.com/rust-lang/rust/issues/23314) on github.
fn main() {
    let me = Person::new("My own self");
    me.greet();
}
