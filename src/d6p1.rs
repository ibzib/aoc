use std::io;
use std::io::prelude::*;

const D: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_next(pos: (usize, usize), delta: (isize, isize)) -> (usize, usize) {
    return (
        pos.0.overflowing_add_signed(delta.0).0,
        pos.1.overflowing_add_signed(delta.1).0,
    );
}

fn main() {
    // Read input and find position
    let stdin = io::stdin();
    let lines: Vec<Vec<u8>> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut pos_opt: Option<(usize, usize)> = None;
    for (i, line) in lines.iter().enumerate() {
        for (j, &b) in line.iter().enumerate() {
            if b == b'^' {
                pos_opt = Some((i, j));
                break;
            }
        }
        if pos_opt.is_some() {
            break;
        }
    }
    let mut pos = pos_opt.unwrap();

    // Traverse the maze
    let mut d = 0;
    let mut visited = vec![vec![false; width]; height];
    let mut ans = 0;
    'outer: loop {
        if !visited[pos.0][pos.1] {
            visited[pos.0][pos.1] = true;
            ans += 1;
        }
        loop {
            let next = get_next(pos, D[d]);
            if next.0 >= height || next.1 >= width {
                // We've exited the maze
                break 'outer;
            }
            if lines[next.0][next.1] == b'#' {
                d = (d + 1) % 4;
            } else {
                pos = next;
                break;
            }
        }
    }
    println!("{}", ans);
}
