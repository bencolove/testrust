# Child Process
1. stdout/stderr

`process::Output` holds output of the finished process, and `process::Command` is a builder for a child process.

`process:Output.stdout` is a byte array of type `&[u8]` which can be converted into `String` by `String::from_utf8_lossy(&[u8])`

```rust
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}
```