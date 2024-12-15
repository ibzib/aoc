use num_bigint::BigInt;
use std::io;
use std::io::prelude::*;
use std::iter::zip;

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    for l in &lines {
        let p: Vec<i32> = l
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        left.push(p[0]);
        right.push(p[1]);
    }

    left.sort();
    right.sort();

    let mut sum = BigInt::ZERO;

    for t in zip(left, right) {
        let d = t.0 - t.1;
        sum += d.abs();
    }

    println!("{}", sum);
}
