use std::io;
use std::io::prelude::*;

type VisitedMap = Vec<Vec<[bool; 4]>>;

fn new_visited_map(height: usize, width: usize) -> VisitedMap {
    return vec![vec![[false; 4]; width]; height];
}

const D: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_next(pos: (usize, usize), delta: (isize, isize)) -> (usize, usize) {
    return (
        pos.0.overflowing_add_signed(delta.0).0,
        pos.1.overflowing_add_signed(delta.1).0,
    );
}

fn traverse(lines: &Vec<Vec<u8>>, start_pos: (usize, usize), visited: &mut VisitedMap) -> bool {
    let mut pos = start_pos;
    let mut d = 0;
    loop {
        if visited[pos.0][pos.1][d] {
            // We're stuck in a cycle
            return true;
        }
        visited[pos.0][pos.1][d] = true;
        loop {
            let next: (usize, usize) = get_next(pos, D[d]);

            if next.0 >= lines.len() || next.1 >= lines[0].len() {
                // We've exited the maze
                return false;
            }
            if lines[next.0][next.1] == b'#' {
                d = (d + 1) % 4;
            } else {
                pos = next;
                break;
            }
        }
    }
}

fn main() {
    // Read input and find position
    let stdin = io::stdin();
    let mut lines: Vec<Vec<u8>> = stdin
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
    let pos = pos_opt.unwrap();

    // Traverse the maze
    let mut visited = new_visited_map(height, width);
    traverse(&lines, pos, &mut visited);

    let mut ans = 0;
    // Don't worry too much about optimization here. You only need to try putting obstructions on spaces the guard would normally visit.
    for (i, line) in visited.iter().enumerate() {
        for (j, &v) in line.iter().enumerate() {
            let vis = v.iter().any(|&b| b);
            if vis && (i, j) != pos {
                let mut visited_obstruction = new_visited_map(height, width);
                lines[i][j] = b'#';
                if traverse(&lines, pos, &mut visited_obstruction) {
                    ans += 1;
                }
                lines[i][j] = b'.';
            }
        }
    }
    println!("{}", ans);
}
