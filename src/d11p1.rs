use std::io;

// We could easily optimize this by memoizing results, but it's not necessary for small parameters.
fn count_stones(x: i64, blinks: i32) -> i64 {
    if blinks == 0 {
        return 1;
    }
    if x == 0 {
        return count_stones(1, blinks - 1);
    }
    let s = x.to_string();
    if s.len() % 2 == 0 {
        let left = s[..s.len() / 2].parse::<i64>().unwrap();
        let right = s[s.len() / 2..].parse::<i64>().unwrap();
        return count_stones(left, blinks - 1) + count_stones(right, blinks - 1);
    }
    return count_stones(2024 * x, blinks - 1);
}

fn main() {
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let mut ans = 0;
        for x in line.split(" ").map(|s: &str| s.parse::<i64>().unwrap()) {
            ans += count_stones(x, 25);
        }
        println!("{}", ans);
    }
}
