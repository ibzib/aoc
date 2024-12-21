use std::io;
use std::io::prelude::*;

fn main() {
    let d = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let word = "XMAS";

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut ans = 0;
    for i in 0..height {
        for j in 0..width {
            for &(di, dj) in &d {
                let mut ci = i;
                let mut cj = j;
                let mut match_len = 0;
                for c in word.chars() {
                    let actual = lines[ci].chars().nth(cj).unwrap();
                    if actual == c {
                        match_len += 1;
                    } else {
                        break;
                    }
                    let ni = ci.checked_add_signed(di as isize);
                    match ni {
                        None => {
                            break;
                        }
                        Some(y) => {
                            if y >= height {
                                break;
                            }
                            ci = y;
                        }
                    }
                    let nj = cj.checked_add_signed(dj as isize);
                    match nj {
                        None => {
                            break;
                        }
                        Some(x) => {
                            if x >= width {
                                break;
                            }
                            cj = x;
                        }
                    }
                }
                if match_len == word.len() {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
