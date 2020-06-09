# Cargo
---

## Project Structure
```
foo  
├── Cargo.toml  
└── src  
    ├── main.rs
    └── bin
        └── my_other_bin.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs
build.rs
```
1. the default binary `main.rs`,
2. and a second one `my_other_bin` by file `src/bin/my_other_bin.rs`. Can be built/run by   
`$ cargo build/run --bin my_other_bin`
3. `tests` are located in `/tests`, run by  
`$ cargo test`
4. `build.rs` default build script or otherwise designated in `Cargo.toml.[package].build` 

## Dependencies
in `Cargo.toml`
```
[package]
...

[dependencies]
clap = "^2.27.1"
rand = { git = "https://github.com/rust-lang-nursery/rand" }
bar = { path = "../bar" }
```

### Use other caret dependencies
When define an entry in `Cargo.toml`'s `[dependencies]`, an dependent caret has to be:
1. imported first as `extern create clap`
2. used with fully-quailified path or shorten by `use caret::path::module`

---

## Profiles
By design, `cargo` has two profiles, namely:
1. dev
2. release

Activating them by `cargo build` and `cargo build --release` respectively.

Values for these profiles can be configured and overwritten in `Cargo.toml`:
```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Publish a Crate to crates.io

## Installing Binaries from crates.io
ONLY packages with **binary** target can be installed locally.
A **binary** target is executable program created if there is **src/main.rs** or any other file specified as **binary** as opposed to a non runnable library target which is intended for integration into other programs.

`cargo install <bin-name>` by default stores the binaries in it's installation root's **bin** fold whichi is probably *$HOME/.cargo/bin* 