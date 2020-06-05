# Cargo

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