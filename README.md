# rust-cat

The first utility you will build is called rust-cat, a variant of the UNIX tool cat. This tool looks through a file, and prints everything it reads.

Here is how we use rust-cat:

```text
WSL rust-cat on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
❯ cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-cat`
Usage: rust-cat [files ...]

WSL rust-cat on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
[😅 ERROR]❯ cargo r Cargo.toml
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-cat Cargo.toml`
[package]
name = "rust-cat"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

WSL rust-cat on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
❯

```
