use std::io::{self, BufRead};

fn get_sums(lines: Vec<String>) -> Vec<i64> {
    let mut sums = Vec::new();
    let nums = lines.iter().map(|l| match l.parse::<i64>() {
        Ok(l) => l,
        Err(_) => 0,
    });
    let mut acc = 0;
    for i in nums {
        if i == 0 {
            sums.push(acc);
            acc = 0;
        } else {
            acc += i;
        }
    }
    sums
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut sums = get_sums(lines);
    sums.sort();
    sums.reverse();
    println!("{}", sums.iter().take(3).sum::<i64>())
}
