use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines: Vec<Vec<u8>> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().as_bytes().to_vec())
        .collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut ans = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if lines[i][j] == b'A' {
                let ul = lines[i - 1][j - 1];
                let ll = lines[i + 1][j - 1];
                let ur = lines[i - 1][j + 1];
                let lr = lines[i + 1][j + 1];
                let down = (ul == b'M' && lr == b'S') || (ul == b'S' && lr == b'M');
                let up = (ll == b'M' && ur == b'S') || (ll == b'S' && ur == b'M');
                if down && up {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
