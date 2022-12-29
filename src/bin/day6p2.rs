use std::{collections::HashSet, fs::read_to_string};

fn check(i: usize, w: &[char]) -> Option<usize> {
    let set: HashSet<char> = w.iter().map(|c| c.to_owned()).collect();

    match set.len() == 14 {
        true => Some(i + 14),
        false => None,
    }
}

fn main() {
    let signal = read_to_string("./input/day6.txt").expect("Could not find input file");

    let signal_chars = signal.chars().collect::<Vec<_>>();

    let chars_before_code = signal_chars
        .windows(14)
        .enumerate()
        .filter_map(|(i, w)| check(i, w))
        .next()
        .unwrap();

    println!("{}", chars_before_code);
}
