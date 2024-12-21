use num_bigint::BigInt;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();

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

    let mut count = HashMap::new();
    for x in right {
        let k = count.entry(x).or_insert(0);
        *k += 1;
    }

    let mut sum = BigInt::ZERO;
    for x in left {
        let k = *count.entry(x).or_default();
        sum += x * k;
    }

    println!("{}", sum);
}
