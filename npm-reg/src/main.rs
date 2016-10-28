extern crate tiny_http;
extern crate rustc_serialize;
//extern crate sha1;

// web hosting: (docs at https://frewsxcv.github.io/tiny-http/tiny_http/index.html )
use tiny_http::{Server, Method, Request, Response, Header};
use std::sync::Arc;
use std::thread;

type ByteResult = Response<std::io::Cursor<Vec<u8>>>;

// sha-sum:
//use sha1::Sha1;

// io
use std::fs::File;
use std::io::{Write, Read, BufWriter};

type IoResult = std::io::Result<()>;

// json
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder, Json, BuilderError};
use rustc_serialize::base64::{FromBase64};

// Pull out the request only-parts of a request
// so we can hold on to the real `Request` for a response
struct DetachedRequest {
    method: Method,
    url: String,
    headers: Vec<Header>,
    body_length: usize,
    remote_addr: Box<std::net::SocketAddr>,
    request_body: Vec<u8>,
}

impl DetachedRequest {
    pub fn from_request(rq: &mut Request) -> DetachedRequest {
        let mut body_in_mem: Vec<u8> = Vec::new();
        rq.as_reader().read_to_end(&mut body_in_mem).expect("Body could not be read");
                                                 // this is not good, but will have to do for now,
                                                 // until I work out the ownership of as_reader()
        let blen = match rq.body_length() {
            Some(len) => len,
            _ => 0,
        };

        DetachedRequest {
            method: rq.method().clone(),
            url: rq.url().to_owned(),
            headers: rq.headers().to_vec(),
            body_length: blen,
            remote_addr: Box::new(*rq.remote_addr()), // note to learner: this heap-allocates the address and will deallocate on drop
            request_body: body_in_mem,
        }
    }
}

fn main() {
    let server = Arc::new(Server::http("0.0.0.0:9975").unwrap());
    println!("Now listening on port 9975");

    loop {
        let server = server.clone();

        let join_handle = thread::spawn(move || {
            for mut rq in server.incoming_requests() {
                let dtrq = DetachedRequest::from_request(&mut rq);
                let _ = rq.respond(handle_request(dtrq));
            }
        });

        match join_handle.join() {
            Ok(_) => println!("Thread died with no panic?!"),
            Err(e) => {
                if let Some(e) = e.downcast_ref::<&'static str>() {
                    println!("Thread panic: {}", e);
                } else {
                    println!("Thread panic: {:?}", e);
                }
            }
        }
    }
}

fn handle_request(rq : DetachedRequest) -> ByteResult {
    match rq.method {
        Method::Put => receive_publish(rq),
        Method::Get => deliver_stored(rq),
        _ => {
            println!("Unsupported HTTP verb {}", rq.method);
            Response::from_string("Unsupported request").with_status_code(400)
        }
    }
}

static ATTACH_DIR: &'static str = "-";

fn receive_publish(rq: DetachedRequest) -> ByteResult {
    println!("PUT -- client: {:?}, url: {:?}, headers: {:?}, body length: {:?}",
             rq.remote_addr,
             rq.url,
             rq.headers,
             rq.body_length
            );

    if rq.body_length < 2 {
        return user_error("Invalid post body");
    }
    let pkg = decode_json_from_bytes(rq.request_body).unwrap();

    // get the package id (which is the file-system safe name?)
    let pkg_id = match pkg.find("_id") {
        Some(id) => id.as_string().unwrap(),
        _ => {
            return user_error("Package has no ID");
        }
    };

    // store the package attachments (this should be AFTER validating the version)
    if let Some(ref attachments) = pkg.find("_attachments") {
        if let Some(a_map) = attachments.as_object() {
            // attachments are a BTreeMap
            for file_name in a_map.keys() {
                if let Some(ref data) = pkg.find_path(&vec!["_attachments",&file_name,"data"]) {
                    let target_path = (vec![pkg_id, ATTACH_DIR, file_name]).join("/");
                    println!("Writing to path {}", target_path);

                    if let Err(e) = write_base64_to_file(&data.to_string(), &target_path) {
                        println!("Failed to write attachment: {:?}", e);
                        return internal_error("Could not store attachments");
                    }
                }
            }
        }
    }

    // write the package.json file out. TODO: this should remove '_attachments' and
    // all other '_...' fields, then merge the new version into existing versions.
    let meta_file_path = (vec![pkg_id, "meta.json"]).join("/");
    if let Err(e) = write_to_file(json_to_bytes(&pkg), &meta_file_path) {
        println!("Failed to write meta '{}': {:?}", meta_file_path, e);
        return internal_error("Could not store attachments");
    }

    return Response::from_string("{}");
}

fn deliver_stored(rq: DetachedRequest) -> ByteResult {
    println!("GET -- client: {:?}, url: {:?}, headers: {:?}, body length: {:?}",
             rq.remote_addr,
             rq.url,
             rq.headers,
             rq.body_length
            );

    // We see if the file exists, in which case it's directly delivered.
    // Otherwise we try adding '/meta.json' to it and deliver that
    // If neither, we 404.
    let direct_path = rq.url.trim_left_matches('/');
    let meta_path = vec![direct_path, "meta.json"].join("/");

    println!("Looking in {:?} or {:?}", direct_path, meta_path);

    if file_exists(&direct_path) {
        println!("Found direct");
        return get_file_result(&direct_path);
    } else if file_exists(&meta_path) {
        println!("Found meta");
        return get_file_result(&meta_path);
    } else {
        println!("Not found");
        return not_found();
    }
}

fn file_exists(path: &str) -> bool {
    let p = std::path::Path::new(&path);
    return p.exists() && p.is_file();
}
fn get_file_result(path: &str) -> ByteResult {
    match load_file_bytes(path) {
        Ok(buf) => Response::from_data(buf),
        _ => internal_error("Could not load file"),
    }
}
fn load_file_bytes(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = try!(File::open(path));
    let mut buf: Vec<u8> = Vec::new();
    let _ = try!(file.read_to_end(&mut buf));
    return Ok(buf);
}

fn internal_error(msg: &str) -> ByteResult { Response::from_string(msg).with_status_code(500) }
fn user_error(msg: &str) -> ByteResult { Response::from_string(msg).with_status_code(400) }
fn not_found() -> ByteResult { Response::from_string("{}").with_status_code(404) }

fn write_base64_to_file(b64str: &str, target: &str) -> IoResult {
    let slice: &str = b64str.trim_matches('"');
    return write_to_file(
        slice.from_base64().expect("invalid base64 string"),
        target
    );
}

fn create_dir_if_needed(target: &str) -> IoResult {
    let path = std::path::Path::new(target);
    if let Some(dir) = path.parent() {
        if let Some(dstr) = dir.to_str() {
            try!(std::fs::create_dir_all(dstr));
        }
    }
    return Ok(());
}

fn write_to_file(data: Vec<u8>, target: &str) -> IoResult {
    try!(create_dir_if_needed(target));

    let f = try!(File::create(target));
    let mut writer = BufWriter::new(f);
    try!(writer.write(&data));

    return Ok(());
}

fn decode_json_from_bytes(data: Vec<u8>) -> Result<Json, BuilderError> {
    return json::Json::from_str(std::str::from_utf8(&data).unwrap());
}

/// Pretty print the supplied structure, and return the utf-8 bytes
fn json_to_bytes<T: Encodable>(thing: &T) -> Vec<u8> {
    let mut encoded = String::new();
    { // scope for the borrowing of `encoded` by `new_pretty`
        let mut encoder = Encoder::new_pretty(&mut encoded);
        thing.encode(&mut encoder).expect("JSON encode error");
    }
    return encoded.into_bytes();
}

