# Child Process
1. ::process::Command - process builder
1. stdout/stderr

## std::process::Command
`Command::new(&path)` builds a process with defaults:
1. NO passed **ARGUMENTS**, 
    1. `.arg(&OsStr)` to append one argument, 
    2. `.args(&[&OsStr])` to append multiples,
2. Inherit parent's **ENVIRONMENTS**, 
    1. `.env(&OsStr, &OsStr)`
    2. `.envs(&HashMap)` `let envs: HashMap<String, String> = env::vars().filter(|&(ref k, v)| k=="TERM").collect(); command.envs(&envs);`
    3. `.env_remove(&OsStr)`
    4. `.env_clear()`
3. Inherit parent's **WORKING DIRECTORY**, `current_dir(&OsStr)` to change it,
4. Inherit parent's **STDIN/STDOUT/STDERR** for `spawn()`, `status()`, pipes created for `.output()`:
    1. `Stdio::inherit()` for `.spawn()` and `.status()`
    2. `piped()` for `.output()`
    1. `Command::stdin(::process::Stdio::piped())` to create new pipe between them
    2. `Command::stdout(::process::Stdio::null())`

`Command` can be consumed by:
    1. `spawn()` -> `Result<Child>`
    2. `status()` -> `Result<ExitStatus>`
    3. `output()` -> `Result<Output>`

## ::process::Child
A `process::Child` is returned from `.spawn()`,
```rust
struct Child {
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
}

impl Child {
    fn kill(&mut self) -> Result<()>;
    fn id(&self) -> u32;
    fn wait(&mut self) -> Result<ExitStatus>;
    fn try_wait(&mut self) -> Result<Option<ExitStatus>>;
    fn wait_with_output(self) -> Result<Output>;
}
```

## ::process::ExitStatus
`process::Child::wait()` -> `io::Result<procss::ExitStatus>`
`Command::status()` -> `io::Result<procss::ExitStatus>`
    1. `ExitStatus::success()` -> `bool`
    2. `ExitStatus::code()` -> `Option<i32>`

## ::process::Output
`procss::Child::wait_with_output()` -> `io::Result<process::Output>`
`Command::output()` -> `io::Result<process::Output>`
```rust
struct Output {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}
```
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

fn run_dependently() -> Vec<u8> {

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };

    output.stdout
}
```