// a quick test of HTTP over a plain TCP socket.

use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;

static SAMPLE_REQUEST: &'static str =
r#"GET http://www.purple.com/ HTTP/1.1
Host: www.purple.com
Accept: text/html

"#;


fn main() {
    let mut result: Vec<u8> = Vec::new();
    let mut stream = TcpStream::connect("www.purple.com:80").unwrap();
        // This is equivalent to `TcpStream::connect("153.104.63.227:80")` due to the
        // std::net::ToSocketAddrs trait.

    stream.set_read_timeout(Some(Duration::new(5,0)));

    if let Err(e) = stream.write(&(SAMPLE_REQUEST.to_string().into_bytes())) {
        println!("Failed to write socket:{:?}", e);
        return;
    }

    let mut buf = &mut[0u8;1024];
    while let Ok(len) = stream.read(buf) {
        if len < 1 {break;}
        result.extend(buf[0..len].iter().cloned()); // Does the equivalent of `result.push(&buf[0..len]);`
                                                    // but copies the values from the buffer into
                                                    // the result vector.
    }

    let result_str = String::from_utf8_lossy(&result);
    println!("{}", &result_str);

    stream.shutdown(std::net::Shutdown::Both).expect("Could not close connection");
}
