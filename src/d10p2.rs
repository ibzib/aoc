use std::io;
use std::mem;

fn bfs(map: &Vec<Vec<u32>>, trailhead: (usize, usize)) -> i32 {
    let m = map.len();
    let n = map[0].len();

    let mut cur_queue = vec![trailhead];
    let mut next_queue = vec![];
    let mut res = 0;
    while !cur_queue.is_empty() {
        for p in &cur_queue {
            for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let q = (p.0.wrapping_add_signed(d.0), p.1.wrapping_add_signed(d.1));
                if q.0 < m && q.1 < n && map[q.0][q.1] == map[p.0][p.1] + 1 {
                    next_queue.push(q);
                    if map[q.0][q.1] == 9 {
                        res += 1;
                    }
                }
            }
        }
        cur_queue.clear();
        mem::swap(&mut cur_queue, &mut next_queue);
    }
    // dbg!(trailhead, res);
    return res;
}

fn main() {
    let map: Vec<Vec<u32>> = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                sum += bfs(&map, (i, j));
            }
        }
    }
    println!("{}", sum);
}
