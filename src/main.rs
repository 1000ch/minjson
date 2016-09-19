use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut buffer = String::new();
    for line in stdin.lock().lines() {
        buffer.push_str(&line.unwrap());
    }

    let mut minified = String::new();
    for c in String::from(buffer).chars() {
        if c != ' ' {
            minified.push(c);
        }
    }

    print!("{}", minified);
}
