extern crate tiny_http;
extern crate sha1;

// web hosting:
use std::sync::Arc;
use std::thread;

// sha-sum:
//use sha1::Sha1;

// io
use std::io::Read;
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

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
    let server = Arc::new(tiny_http::Server::http("0.0.0.0:9975").unwrap());
    println!("Now listening on port 9975");

    let mut handles = Vec::new();

    //for _ in 0 .. 4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for mut rq in server.incoming_requests() {
                if let Some(length) = rq.body_length() {
                    if length > 0 {
                        let mut reader = rq.as_reader();
                        write_to_file(&mut reader, "published.json");
                    }
                }

                println!("received request! method: {:?}, url: {:?}, headers: {:?}, body length: {:?}",
                         rq.method(),
                         rq.url(),
                         rq.headers(),
                         rq.body_length()
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

fn write_to_file(reader: &mut Read, target: &str) {
    let f = File::create(target).expect("could not create file");
    {
        let mut writer = BufWriter::new(f);
        let mut buf = &mut[0u8;1024];

        while let Ok(len) = reader.read(buf) {
            if len < 1 {break;}
            writer.write(&buf[0..len]).expect("File write failed");
        }
    }
}
