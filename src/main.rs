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

fn main() {
    // 1. Check argv[] status
    let argv: Vec<String> = env::args().collect();
    if argv.len() == 1 {
        eprintln!("Usage: rust-cat [files ...]");
        std::process::exit(1);
    }

    // 2. Run with Config
    if let Err(e) = rust_cat::get_args().and_then(rust_cat::run) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
