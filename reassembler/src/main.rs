
extern crate rustc_serialize;

use rustc_serialize::json::{self};
use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;

/// This is the content of a single relation
#[derive(RustcDecodable, RustcEncodable)]#[allow(non_snake_case)]
struct Relation {
    SchemaElementGuid: String,
    Child: String,
    Kind: String,
    IsDependent: bool,
}
/// This is the type of the file: `GUID -> [ Relation ]`
type RelationSet = BTreeMap<String, Vec<Relation>>;


fn main() {
    println!("Hello, world!");
    let data_string = read_file_as_string("Relationships.json");

    let decoded: BTreeMap<String, Vec<Relation>> = json::decode(&data_string).expect("JSON file did not match Rust struct");

    //println!("{}", pretty_print(decoded)); // this can take a while ...
    println!("There are {} parents in the relation map", decoded.len());
}

fn read_file_as_string(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("could not open sample file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("could not read sample file");
    return s;
}
/*
fn pretty_print<T: Encodable>(thing: T) -> String {
    let mut encoded = String::new();
    { // scope for the borrowing of `encoded` by `new_pretty`
        let mut encoder = Encoder::new_pretty(&mut encoded);
        thing.encode(&mut encoder).expect("JSON encode error");
    }
    return encoded;
}
*/

