use std::io;
use std::io::prelude::*;
#[macro_use]
extern crate itertools;

fn main() {
    let stdin = io::stdin();
    let lines: Vec<Vec<u8>> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    // You could optimize this by grouping common "frequencies" ahead of time, but input is small so keep it simple.
    let m = lines.len();
    let n = lines[0].len();
    let mut antinode = vec![vec![false; n]; m];
    let mut ans = 0;
    for p in iproduct!(0..m, 0..n) {
        if lines[p.0][p.1] == b'.' {
            // Not an antenna
            continue;
        }
        for q in iproduct!(0..m, 0..n) {
            if p == q {
                // Same coordinates
                continue;
            }
            if lines[p.0][p.1] != lines[q.0][q.1] {
                // Frequencies don't match
                continue;
            }
            // Find antinode.
            //
            // Use wrapping_sub to catch underflow (negatives) in our upper bounds check.
            //
            // Because the loop visits all pairs of points (p, q) in both orders (p, q) and (q, p),
            // each iteration only needs to check one of the two antinodes.
            let a = ((2 * p.0).wrapping_sub(q.0), (2 * p.1).wrapping_sub(q.1));
            // check upper bounds
            if !(a.0 < m && a.1 < n) {
                continue;
            }
            if !antinode[a.0][a.1] {
                // New antinode location found
                ans += 1;
                antinode[a.0][a.1] = true;
            }
        }
    }

    // Print map of antinode locations
    for i in 0..m {
        for j in 0..n {
            let c = if antinode[i][j] { "#" } else { "." };
            print!("{}", c);
        }
        println!();
    }

    // Answer
    println!("{}", ans);
}
