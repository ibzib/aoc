use std::io;
use std::io::prelude::*;

fn is_safe_skip(p: &Vec<i32>, skip: usize) -> bool {
    let first = if skip == 0 { 1 } else { 0 };
    let second = if skip < 2 { 2 } else { 1 };
    let increasing = p[second] > p[first];
    let mut prev = p[first];
    for i in second..p.len() {
        if i == skip {
            continue;
        }
        let x = p[i];
        let inc = x > prev;
        if inc != increasing {
            return false;
        }
        let d = (x - prev).abs();
        if d < 1 || d > 3 {
            return false;
        }
        prev = x;
    }
    return true;
}

fn is_safe(p: &Vec<i32>) -> bool {
    for skip in 0..p.len() {
        if is_safe_skip(p, skip) {
            return true;
        }
    }
    return false;
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let mut ans = 0;
    for l in &lines {
        let p: Vec<i32> = l
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if is_safe(&p) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
