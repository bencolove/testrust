# fs::File

## Open File
The builder:
```rust
use std::fs::OpenOptions;
let file = OpenOptions::new().read(true).open("foo.txt");
```
The `OpenOptions` can set options by chained calls like:
1. `read`
2. `write`
3. `append` (need `flush` for buffered contents or `write` in one go)
4. `truncate` (need `write`)
5. `create` (open or create one, `write` or `append` is needed)
6. `create_new` (create new or fail if existed, `write` or `append` is needed)
7. `open` `OpenOptions.open<P: AsRef<Path>>(&self, path: P) -> io::Result<File, io::Error>`

The `File::open` and `File::create` are just convinient shortcuts for using it:
`File::open` => `read` read-only, fail when not present
`File::create` => `create` write-only, create if absent, truncate if present

## Read
1. read as lines via buffer
```rust

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// consume it
// `io::Lines` is `Iterator`
if let Ok(lines) = read_lines("foo.txt") {
    for line in lines {
        if let Ok(content) = line {
            // line is Option<&str>
        }
    }
}
```
2. read all in one go
```rust
let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
};
// Read the file contents into a string, returns `io::Result<usize>`
let mut s = String::new();
match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why),
    Ok(_) => print!("{} contains:\n{}", display, s),
}
```

3. read in batch ???


## Write
1. write from buffer, non-blocking
```rust
let mut buffer: Vec<u8> = [0; 512];
???
match file.write(LOREM_IPSUM.as_bytes()) {
    Err(why) => panic!("couldn't write to {}: {}", display, why),
    Ok(_) => println!("successfully wrote to {}", display),
}
```
2. write all in one go, blocking
```rust
match file.write_all(LOREM_IPSUM.as_bytes()) {
    Err(why) => panic!("couldn't write to {}: {}", display, why),
    Ok(_) => println!("successfully wrote to {}", display),
}
```

examples:
```rust
`mkdir -p a/b/c`
fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
});

`ls -a`
// fs::read_dir(&path) -> io::Result<Vec<Path>>
match fs::read_dir("a") {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(paths) => for path in paths {
        println!("> {:?}", path.unwrap().path());
    },
}
`rm a/c/e.txt`
// Remove a file, returns `io::Result<()>`
fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
});

`rmdir a/c/d`
// Remove an empty directory, returns `io::Result<()>`
fs::remove_dir("a/c/d").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
});

// unix only
`ln -s ../b.txt a/c/b.txt`
// Create a symbolic link, returns `io::Result<()>`
if cfg!(target_family = "unix") {
    unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
    });
}
```

```rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
```
```rust
static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
```