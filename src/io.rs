use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let lines_iter = stdin.lock().lines().map(|l| l.unwrap());

    let byte_lines: Vec<Vec<u8>> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
}
