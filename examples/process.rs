use std::process::{
    Command, ExitStatus, Output, Stdio
};
use std::io::Result;

fn main() {
    run_with_output("ls -l");
}

fn run_with_output(command: &str) -> Result<String> {
    let tokens = command.split_whitespace();
    // only collections can `.iter()`, `.iter_mut()` or `.into_iter()`
    // for-loop over Iterators consumes them
    for token in tokens {
        println!("{:?}", token);
    }

    let tokens = command.split_whitespace();
    // convert into Vec
    let cmd_parts = tokens.collect::<Vec<&str>>();
    println!("{:?}", cmd_parts);

    // slice over &Vec
    let (file, args) = (&cmd_parts[0], &cmd_parts[1..]);
    println!("command={}, args={:?}", file, args);

    // run command and wait for output
    // `output()` -> `Result`
    let output = Command::new(file).args(args).output();
    // println!("{:?}", output);
    // `?` `unwrap()` if `OK` or return `Result` if `Err`
    let success = output?.status.success();
    println!("{}", success);

    

    // return Ok(String::from("abc"));
    return Ok("abc".to_owned());
}