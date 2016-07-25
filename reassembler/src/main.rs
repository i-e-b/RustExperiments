
extern crate rustc_serialize;

use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder, Json};
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

#[derive(RustcDecodable, RustcEncodable)]#[allow(non_snake_case)]
struct Instance {
    Meta: Vec<NameValuePair>,
    Data: Vec<TypedNameValuePair>,
}

#[derive(RustcDecodable, RustcEncodable)]#[allow(non_snake_case)]
struct NameValuePair {
    Name: String,
    Value: String,
}

#[derive(RustcDecodable, RustcEncodable)]#[allow(non_snake_case)]
struct TypedNameValuePair {
    Name: String,
    SchemaElementGuid: String,
    Value: Option<String>,
    Type: String,
}

/// This is the type of the "Relationships.json" file: `GUID -> [ Relation ]`
type RelationSet = BTreeMap<String, Vec<Relation>>;

/// Type of the "Instances.json" file: `GUID -> Instance`
type InstanceSet = BTreeMap<String, Instance>;


fn main() {
    println!("Reading files into RAM");
    let relations_string = read_file_as_string("Relationships.json");
    let instances_string = read_file_as_string("Instances.json");

    println!("Decoding relations");
    let relations: RelationSet = json::decode(&relations_string).expect("Relationships JSON file did not match Rust struct");

    println!("Decoding instances");
    let instances: InstanceSet = json::decode(&instances_string).expect("Instances JSON file did not match Rust struct");

    println!("done");
    let root_guid = "137ca03d-10d6-4a5c-866d-30313ec8b884";

    println!("There are {} parents in the relation map", relations.len());
    println!("There are {} object data sets in the instances map", instances.len());

    println!("Building tree for {}", root_guid);
    let tree = build_tree(root_guid, &relations, &instances);
    println!("Example {:?}", tree);
}


struct NodeRel {
    parent: String,
    child: String,
    kind: String,
}
struct DecomposedTree {
    root: u32,
    rels: Vec<NodeRel>,
    nodes: Vec<Json>,
}
impl DecomposedTree {
    pub fn new() -> DecomposedTree {
        DecomposedTree {
            root:  0,
            rels:  Vec::new(),
            nodes: Vec::new(),
        }
    }
}



fn build_tree(root_guid: &str, relations: &RelationSet, instances: &InstanceSet) -> Option<Json> {
    // Plan: Build out the {parent,child,kind} relations like TreeSurgeon,
    //       then transform the data elements of the instance data to be key->value pairs,
    //       and finally run the reconstruction from T.S. to get a tree.

    let mut tree = DecomposedTree::new();

    let mut all_nodes : Vec<String> = Vec::new();

    let mut queue = Vec::new();
    queue.push(root_guid);

    while let Some(parent) = queue.pop() {
        if let Some(children) = relations.get(parent) {
            for child in children {
                tree.rels.push(NodeRel{ parent:parent.to_string(), child:child.Child.to_string(), kind:child.Kind.to_string() });
                queue.push(&child.Child);
                all_nodes.push(parent.to_string());
                all_nodes.push(child.Child.to_string());
            }
        }
    }

    all_nodes.sort();
    all_nodes.dedup();

    // Now go and build the tree

    println!("{} nodes, {} rels", all_nodes.len(), tree.rels.len());

    None
}

fn read_file_as_string(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("could not open sample file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("could not read sample file");
    return s;
}

fn pretty_print<T: Encodable>(thing: T) -> String {
    let mut encoded = String::new();
    { // scope for the borrowing of `encoded` by `new_pretty`
        let mut encoder = Encoder::new_pretty(&mut encoded);
        thing.encode(&mut encoder).expect("JSON encode error");
    }
    return encoded;
}


