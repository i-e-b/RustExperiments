extern crate sha1;

use sha1::Sha1;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let p = env::current_dir().unwrap();
    println!("The current directory is {}", p.display());

    let mut f = File::open("sample_file.txt").expect("could not open sample file");
    let mut m = Sha1::new();

    let mut buf = &mut[0u8;64];
    while let Ok(len) = f.read(buf) {
        if len < 1 {break;}
        m.update(&buf[0..len]);
    }
    // online: 943a702d06f34599aee1f8da8ef9f7296031d699
    // nodejs: 943a702d06f34599aee1f8da8ef9f7296031d699
    // this  : 943a702d06f34599aee1f8da8ef9f7296031d699

    println!("{}", m.digest());
}
