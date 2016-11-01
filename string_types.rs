//! `&`      -> this is a 'by reference' sigil.
//!
//! `str`    -> this is an immutable 'string slice', which is Rust's most primitive string type.
//!             It is always UTF-8, and in memory is a Vec<u8>.
//!             [Rust docs page](https://doc.rust-lang.org/std/primitive.str.html)
//!
//! `String` -> this is a 'growable' string that has both a length and capacity, plus some
//!             functions to edit the contents (push,pop,trunc,append...)
//!             [Rust docs page](https://doc.rust-lang.org/std/string/struct.String.html)
//!
//! `&String` is a reference to the growable string, but can be passed anywhere that a `&str` is
//! required -- the compiler will insert the convertion methods for you.


// If you want a global const string, you have to give it the `static` life-span
// Also shows unescaped strings, which start with `r#"` and end with `"#`
// REMEMBER: `const`s are INLINED per use. If you use lots of these, your exe will be huge
const SAMPLE_CONSTANT: &'static str = r#"
This is
a "unescaped"
multi-line string
"#;

// This is a better way for global strings -- this is help in one place in the exe
// and in memory, and gets referenced by address where needed.
static SAMPLE_STATIC: &'static str = "This is a better way to do it";

fn main(){
    let my_str: &'static str = "Hello";
    let pt2 = ", world";

    // Direct conversion:
    let mut string_a = my_str.to_owned();    // these two
    let mut string_b = my_str.to_string();   // are essentially the same

    // Type conversion with `into`
    let mut string_c:String = my_str.into(); // needs the type annotation

    // Create String instance from a slice using a static function
    let mut string_d = String::from(my_str);

    // New editable string and append to it:
    let mut string_e = String::new();
    string_e.push_str(SAMPLE_CONSTANT);

    // Prove that we are editable:
    string_a.push_str(pt2);
    string_b.push_str(pt2);
    string_c.push_str(pt2);
    string_d.push_str(pt2);
    string_e.push_str(pt2);

    // Output
    println!("{}", string_a);
    println!("{}", string_b);
    println!("{}", string_c);
    println!("{}", string_d);
    println!("{}", string_e);

    // Converting back to a slice:
    let immutable_string        = &string_a; // This is `&String` type
    let slice_string     : &str = &string_a; // This is `&str` type

    // Converting to and from bytes:
    let bytes: Vec<u8> = SAMPLE_STATIC.to_string().into_bytes();
    let decoded        = String::from_utf8(bytes).expect("could not decode string bytes");

    println!("{} = {}", immutable_string, slice_string);
    println!("{} = {}", SAMPLE_STATIC, &decoded);
}
