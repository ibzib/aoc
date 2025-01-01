use std::collections::HashMap;
use std::io;

struct StoneCounter {
    cache: HashMap<(i64, i32), i64>,
}

impl StoneCounter {
    fn new() -> StoneCounter {
        StoneCounter {
            cache: HashMap::new(),
        }
    }

    fn count_stones(&mut self, x: i64, blinks: i32) -> i64 {
        if blinks == 0 {
            // Recursive base case, no point in caching it.
            return 1;
        }

        let k = (x, blinks);

        // Fetch existing answer from cache
        if let Some(&n) = self.cache.get(&k) {
            return n;
        }

        // Compute new answer and cache it
        let s = x.to_string();
        let n = if x == 0 {
            self.count_stones(1, blinks - 1)
        } else if s.len() % 2 == 0 {
            let (ls, rs) = s.split_at(s.len() / 2);
            [ls, rs]
                .iter()
                .map(|ys| ys.parse::<i64>().unwrap())
                .map(|y| self.count_stones(y, blinks - 1))
                .sum()
        } else {
            self.count_stones(2024 * x, blinks - 1)
        };
        self.cache.insert(k, n);
        return n;
    }
}

fn main() {
    let mut counter = StoneCounter::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let mut ans = 0;
        for x in line.split(" ").map(|s: &str| s.parse::<i64>().unwrap()) {
            ans += counter.count_stones(x, 75);
        }
        println!("{}", ans);
    }
}
