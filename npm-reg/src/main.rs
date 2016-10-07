extern crate tiny_http;

use std::sync::Arc;
use std::thread;

const SAMPLE_RESULT: &'static str = r#"
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
    let server = Arc::new(tiny_http::Server::http("0.0.0.0:9975").unwrap());
    println!("Now listening on port 9975");

    let mut handles = Vec::new();

    //for _ in 0 .. 4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for rq in server.incoming_requests() {
                println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                         rq.method(),
                         rq.url(),
                         rq.headers()
                        );
                let response = tiny_http::Response::from_string(SAMPLE_RESULT);
                let _ = rq.respond(response);
            }
        }));
    //}

    for h in handles {
        h.join().unwrap();
    }
}
