extern crate tiny_http;
extern crate rustc_serialize;
//extern crate sha1;

// web hosting: (docs at https://frewsxcv.github.io/tiny-http/tiny_http/index.html )
use tiny_http::{Server, Method, Request, Response, Header};
use std::sync::Arc;
use std::thread;

use std::cell::RefCell;
use std::rc::Rc;

type ByteResult = Response<std::io::Cursor<Vec<u8>>>;

// sha-sum:
//use sha1::Sha1;

// io
use std::io::Read;
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

// json
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder, Json, BuilderError};
use rustc_serialize::base64::{self, FromBase64};

// Pull out the request only-parts of a request
// so we can hold on to the real `Request` fro a response
struct DetachedRequest {
    method: Method,
    url: String,
    headers: Vec<Header>,
    body_length: Option<usize>,
    remote_addr: Box<std::net::SocketAddr>,
    request_body: Vec<u8>,
}
impl DetachedRequest {
    pub fn from_request(rq: &mut Request) -> DetachedRequest {
        let mut body_in_mem: Vec<u8> = Vec::new();
        rq.as_reader().read_to_end(&mut body_in_mem); // this is not good, but will have to do for now,
                                                 // until I work out the ownership of as_reader()

        DetachedRequest {
            method: rq.method().clone(),
            url: rq.url().to_owned(),
            headers: rq.headers().to_vec(),
            body_length: rq.body_length(),
            remote_addr: Box::new(*rq.remote_addr()), // note to learner: this heap-allocates the address and will deallocate on drop
            request_body: body_in_mem,
        }
    }
}

static SAMPLE_RESULT: &'static str = r#"
{
  "_id" : "rusty-package",
  "_rev" : "62-d921946c06d3ef9f327d6a3014a94b22",
  "name" : "rusty-package",
  "description" : "A fake package",
  "dist-tags" : {},
  "versions" : {
    "0.0.1": {
    "name" : "rusty-package",
      "description" : "a thing",
      "version" : "0.0.1",
      "author" : { "name" : "ieb" },
      "main" : "index.js",
      "dist" : {
        "tarball" : "http://localhost:9975/rusty-package/-/rusty-package-0.0.1.tgz",
        "shasum" : "dbd05eebe43c52007c83e0b1dfb7b7c6607068fb"
      }
    }
  }
}
"#;

fn main() {
    let server = Arc::new(Server::http("0.0.0.0:9975").unwrap());
    println!("Now listening on port 9975");

    let mut handles = Vec::new();

    //for _ in 0 .. 4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for mut rq in server.incoming_requests() {
                let dtrq = DetachedRequest::from_request(&mut rq);
                let _ = rq.respond(handle_request(dtrq));
            }
        }));
    //}

    for h in handles {
        h.join().unwrap();
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
    if let Some(length) = rq.body_length {
        if length > 0 {
            let pkg = decode_json_from_bytes(rq.request_body).unwrap();

            // get the package id (which is the file-system safe name?)
            let pkg_id = match pkg.find("_id") {
                Some(id) => id.as_string().unwrap(),
                _ => {
                    return Response::from_string("Package has no ID").with_status_code(400);
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

                            // TODO: need to create the lead-up folders if they don't already exist
                            write_base64_to_file(&data.to_string(), &target_path);
                        }
                    }
                }
            }
        }
    }

    println!("received request! method: {:?}, url: {:?}, headers: {:?}, body length: {:?}",
                rq.method,
                rq.url,
                rq.headers,
                rq.body_length
            );

    return Response::from_string("{}");
}

fn deliver_stored(rq: DetachedRequest) -> ByteResult {
    return Response::from_string(SAMPLE_RESULT);
}

fn write_base64_to_file(b64str: &str, target: &str) {
    let slice: &str = b64str.trim_matches('"');
    write_to_file(
        slice.from_base64().expect("invalid base64 string"),
        target
    );
}

fn write_to_file(data: Vec<u8>, target: &str) {
    let f = File::create(target).expect("could not create file");
    let mut writer = BufWriter::new(f);
    writer.write(&data).expect("File write failed");
}

fn decode_json_from_bytes(data: Vec<u8>) -> Result<Json, BuilderError> {
    return json::Json::from_str(std::str::from_utf8(&data).unwrap());
}
