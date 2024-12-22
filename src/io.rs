use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    // If you need all the lines in memory
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    // If you don't
    let lines_iter = stdin.lock().lines().map(|l| l.unwrap());
}
