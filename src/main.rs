extern crate atty;

pub mod lib;
pub mod parser;
pub mod translator;

use std::io;
use std::io::BufRead;

fn read_stdin_to_str() -> Result<String, std::io::Error> {
    let mut result: String = String::from("");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                result.push_str(&line);
                result.push_str(&"\n");
            },
            Err(e) => return Err(e)
        }
        
    }
    result.push_str(&"\n");
    Ok(result)
}

fn main() -> io::Result<()> {
    if atty::is(atty::Stream::Stdin) {
        println!("Incorrect usage. You need to provide a md file on stdin, for example:");
        println!("$ cat input.md | prose > output.html");
        std::process::exit(1)
    }

    match read_stdin_to_str() {
        Ok(md) => {
            println!("{}", markdown(&md));
        },
        Err(e) => return Err(e)
    }
    Ok(())
}