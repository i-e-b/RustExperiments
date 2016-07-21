
extern crate rustc_serialize;

use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};
use std::fs::File;
use std::io::Read;
use std::env;

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

    println!("Decoding values:");
    println!("Working dir is {}", env::current_dir().unwrap().display()); // expect to run from the cargo base
    let data_string = read_file_as_string("sample.json");

    // Decoding as a known type (extra fields will get skipped, missing fields cause errors)
    let decoded: User = json::decode(&data_string).expect("JSON file did not match Rust struct");
    println!("The user {} has posted {} times and {} burgers",
             decoded.name, decoded.post_count,
             if decoded.likes_burgers {"likes"} else {"dislikes"}
             );

    // Decoding as a generic `Json` struct
    // The `let Ok(var_name) = Container<T>` is a deconstruting, assignment, and pattern match all
    // in one. Looks a bit weird, but saves some boiler plate.
    if let Ok(dyn_json) = json::Json::from_str(&data_string) { // get dynamic `Json` struct from a string
        if let Some(ref user_name) = dyn_json.find("name") {     // equivalent of JS `dyn_json["name"]`
            println!("Username found: {}", user_name);
        }
        if let Some(ref upvotes) = dyn_json.find_path(&vec!["stats","upvotes"]) { // like `dyn_json.stats.upvotes`
            println!("Upvotes: {}", upvotes);
        }
        // See https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/enum.Json.html
    }
}

//----------------------------------------------------------------------------------------------------//

fn read_file_as_string(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("could not open sample file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("could not read sample file");
    return s;
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
