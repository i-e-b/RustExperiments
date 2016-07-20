
extern crate rustc_serialize;

use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

#[derive(RustcDecodable, RustcEncodable)]
struct Photo {
    url: String,
    dimensions: (u32,u32),
}

#[derive(RustcDecodable, RustcEncodable)]
struct User {
    name: String,
    post_count: u32,
    likes_burgers: bool,
    avatar: Option<Photo>,
}

fn main() {
    println!("Encoding some values:");
    println!("    {:?}", json::encode(&42));
    println!("    {:?}", json::encode(&vec!["to","be", "or", "not", "to", "be"]));
    println!("    {:?}", json::encode(&Some(true)));

    let user = sample_user();
    println!("    {:?}", json:: encode(&user));

    println!("Pretty printer:");
    let pretty = pretty_print(user);
    println!("{}", pretty);
}

// note the generic type taking the encodable trait
fn pretty_print<T: Encodable>(thing: T) -> String {
    let mut encoded = String::new();
    { // scope for the borrowing of `encoded` by `new_pretty`
        let mut encoder = Encoder::new_pretty(&mut encoded);
        thing.encode(&mut encoder).expect("JSON encode error");
    }
    return encoded;
}

fn sample_user() -> User {
    User {
        name: String::from("Joe B. Userface"),
        post_count: 100u32,
        likes_burgers: true,
        avatar: Some(Photo{
            url: String::from("http://purple.com"),
            dimensions: (128u32, 128u32),
        }),
    }
}
