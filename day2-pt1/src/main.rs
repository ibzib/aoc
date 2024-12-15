use std::io;
use std::io::prelude::*;

fn is_safe(p: Vec<i32>) -> bool {
    let increasing = p[1] > p[0];
    let mut prev = p[0];
    for &x in &p[1..] {
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

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let mut ans = 0;
    for l in &lines {
        let p: Vec<i32> = l
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if is_safe(p) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
