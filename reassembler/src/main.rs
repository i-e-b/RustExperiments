//! A simple but non-trivial app that reads a pair of large JSON documents
//! And composes parts of them into a new tree structure.

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


#[derive(Debug)]
struct NodeRel {
    child: String,
    kind: String,
}
#[derive(Debug)]
struct DecomposedTree {
    root: u32,
    rels: BTreeMap<String, Vec<NodeRel>>,  // parent id -> [child id * kind]
    nodes: BTreeMap<String, Json>, // id -> json fragment
}
impl DecomposedTree {
    pub fn new() -> DecomposedTree {
        DecomposedTree {
            root:  0,
            rels:  BTreeMap::new(),
            nodes: BTreeMap::new(),
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
                merge(&mut tree.rels, parent.to_string(), NodeRel{ child:child.Child.to_string(), kind:child.Kind.to_string() });

                queue.push(&child.Child);
                all_nodes.push(parent.to_string());
                all_nodes.push(child.Child.to_string());
            }
        }
    }

    all_nodes.sort();
    all_nodes.dedup();

    // Fetch the nodes we need
    for node_ref in all_nodes.iter() {
        let j = convert_to_key_value(instances.get(node_ref).unwrap());

        println!("{}", pretty_print(&j));
        tree.nodes.insert(node_ref.to_string(), j);
    }

    // Now go and build the tree
    let outp = compose_rec(root_guid, tree);

    println!("{} nodes, {} rels", all_nodes.len(), tree.rels.len());

    return outp;
}

fn compose_rec(current_id: &String, data: DecomposedTree) -> Json {
    // recurse down the children, then merge into the current node, return
    if let Some(rels) = data.rels.get(current_id) {
        let mut blob: BTreeMap<String, Json> = BTreeMap::new();
        for rel in rels { // rels: BTreeMap<String, Vec<NodeRel>>,  // parent id -> [child,kind]
            let j = compose_rec(&rel.child, data);
            blob.insert
        }
    }

    return Json::Null;
}

fn merge<TK:Ord, TV>(map: &mut BTreeMap<TK, Vec<TV> >, key: TK, value: TV) {
    // some of the mutable borrow stuff in here is weird.
    // There is probably a better way to do this I haven't found.
    { // start scope
        let v = map.get_mut(&key); // mutable borrow of `map`, by taking mutable value
        if let Some(list) = v {
            list.push(value);
            return;
        }
    } // end of mutable borrow, so we can now take another:
    map.insert(key, vec![value]);
}

fn convert_to_key_value(thing: &Instance) -> Json {
    let mut kvs: BTreeMap<String, Json> = BTreeMap::new();
    let mut meta: BTreeMap<String, Json> = BTreeMap::new();

    for pair in thing.Meta.iter() {
        meta.insert(pair.Name.to_string(), Json::String(pair.Value.to_string()));
    }

    kvs.insert("_meta".to_string(), Json::Object(meta));

    for pair in thing.Data.iter() {
        match pair.Value {
            Some(ref value) => kvs.insert(pair.Name.to_string(), Json::String(value.to_string())),
            None            => kvs.insert(pair.Name.to_string(), Json::Null),
        };
    }

    return Json::Object(kvs);
}

fn read_file_as_string(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("could not open sample file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("could not read sample file");
    return s;
}

fn pretty_print<T: Encodable>(thing: &T) -> String {
    let mut encoded = String::new();
    { // scope for the borrowing of `encoded` by `new_pretty`
        let mut encoder = Encoder::new_pretty(&mut encoded);
        thing.encode(&mut encoder).expect("JSON encode error");
    }
    return encoded;
}


