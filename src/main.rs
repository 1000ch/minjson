use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut input = String::new();
    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
    }

    let mut chars = Vec::new();
    for c in String::from(input).chars() {
        if c != ' ' {
            chars.push(c);
        }
    }

    let string = chars.into_iter().collect::<String>();
    stdout.lock().write(string.as_bytes());
}
