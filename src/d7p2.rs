use std::io;
use std::io::prelude::*;

fn solve(target: i64, equation: &Vec<i64>, i: usize) -> bool {
    if i == equation.len() - 1 {
        return target == equation[i];
    }
    let mult = target % equation[i] == 0 && solve(target / equation[i], equation, i + 1);
    let add = equation[i] <= target && solve(target - equation[i], equation, i + 1);
    // Concatenation operator ||
    let target_str = target.to_string();
    let operand_str = equation[i].to_string();
    let conc = if target_str.len() > operand_str.len() && target_str.ends_with(&operand_str) {
        let next = target_str[..target_str.len() - operand_str.len()]
            .parse::<i64>()
            .unwrap();
        solve(next, equation, i + 1)
    } else {
        false
    };
    return mult || add || conc;
}

fn main() {
    let stdin = io::stdin();
    let lines_iter = stdin.lock().lines().map(|l| l.unwrap());
    let mut ans: i64 = 0;
    for line in lines_iter {
        let tokens: Vec<&str> = line.split(" ").collect();
        let target_str = tokens[0].split(":").collect::<Vec<&str>>()[0];
        let target = target_str.parse::<i64>().unwrap();
        let equation: Vec<i64> = tokens[1..]
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .rev()
            .collect();
        if solve(target, &equation, 0) {
            // Usually the input won't cause an overflow, but check just in case.
            ans = ans.checked_add(target).unwrap();
        }
    }
    println!("{}", ans);
}
