extern crate atty;

pub mod parser;
pub mod translator;

use std::io;
use std::io::BufRead;

type MarkdownText = Vec<MarkdownInline>;

#[derive(Clone, Debug, PartialEq)]
pub enum Markdown {
    Heading(usize, MarkdownText),
    OrderedList(Vec<MarkdownText>),
    UnorderedList(Vec<MarkdownText>),
    Line(MarkdownText),
    Codeblock(String, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownInline {
    Link(String, String),
    Image(String, String),
    InlineCode(String),
    Bold(String),
    Italic(String),
    Plaintext(String),
}

fn markdown(md: &str) -> String {
    match parser::parse_markdown(md) {
        Ok((_, m)) => translator::translate(m),
        Err(_) => String::from("Sorry, this did not seem to work! Maybe your markdown was not well formed, have you hit [Enter] after your last line?"),
    }
}

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