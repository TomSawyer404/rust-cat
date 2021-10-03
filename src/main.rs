//! # Note
//! This is a 'cat' program based on Rust.  
//! @auhtor:    Mrbanana
//! @date:      2021-7-25
//! @license:   The MIT License
//!
//! Usage:
//!
//! ```bash
//! rust-cat [files ...]
//! ```
//!
//! # Knowledge you will learn
//! - How to read a file in Rust
//! - How to get argv[] from enviroment variable

use std::env;
use std::fs::*;
use std::io::*;

fn main() {
    // 1. Get argv[] from env
    let argv: Vec<String> = env::args().collect();
    if argv.len() == 1 {
        eprintln!("Usage: rust-cat [files ...]");
        std::process::exit(1);
    }

    // 2. Open file based on argv[i]
    for i in 1..argv.len() {
        let fd = File::open(argv[i].as_str()).expect("Cannot open file!");

        // 3. Read the file
        let mut content = String::new();
        let mut buf_reader = BufReader::new(fd);
        buf_reader
            .read_to_string(&mut content)
            .expect("Cannot read the file!");

        // 4. Display the content
        print!("{}", content)
    }
}
