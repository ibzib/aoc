use num_bigint::BigInt;
use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut ans = BigInt::ZERO;

    let mut enabled = true;
    for s in lines {
        for c in re.captures_iter(&s) {
            // dbg!(&c);
            if &c[0] == "do()" {
                enabled = true;
            } else if &c[0] == "don't()" {
                enabled = false;
            } else if enabled {
                let x: i32 = c[1].parse().unwrap();
                let y: i32 = c[2].parse().unwrap();
                ans += x * y;
            }
        }
    }

    println!("{}", ans);
}
