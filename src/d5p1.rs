use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Default)]
struct Rules {
    rules: HashMap<i32, HashSet<i32>>,
}

impl Rules {
    fn add(&mut self, before: i32, after: i32) {
        self.rules
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }

    fn compare(&self, a: &i32, b: &i32) -> bool {
        match self.rules.get(b) {
            None => true,
            Some(set) => !set.contains(a),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let mut rules = Rules::default();
    let mut ans = 0;
    for line in lines {
        if line.contains("|") {
            let pages: Vec<i32> = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
            rules.add(pages[0], pages[1]);
        } else if line.contains(",") {
            let pages: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            let sorted = pages.iter().is_sorted_by(|a, b| rules.compare(a, b));
            if sorted {
                ans += pages[pages.len() / 2];
            }
        }
    }
    println!("{}", ans);
}
