use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Pair {
    from: i32,
    to: i32,
}

trait Long {
    fn long(&self) -> Vec<i32>;
}

impl Long for Pair {
    fn long(&self) -> Vec<i32> {
        (self.from..self.to + 1).collect::<Vec<i32>>()
    }
}

fn shared(p: Vec<Pair>) -> Option<bool> {
    let l = p[0].long();
    let r = p[1].long();

    let set_l: HashSet<i32> = l.into_iter().collect();
    let set_r: HashSet<i32> = r.into_iter().collect();

    let u = set_l.union(&set_r).count();

    match (u == set_l.len(), u == set_r.len()) {
        (true, _) => Some(true),
        (_, true) => Some(true),
        _ => None,
    }
}

fn pairs_from_line(line: &str) -> Vec<Pair> {
    let s = line
        .split(',')
        .map(|j| j.split('-').filter_map(|p| p.parse::<i32>().ok()))
        .map(|mut i| Pair {
            from: i.next().unwrap(),
            to: i.next().unwrap(),
        })
        .collect::<Vec<_>>();

    s
}

fn main() {
    let file_lines = read_to_string("./input/day4.txt")
        .expect("Could not find input file")
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<_>>();

    let pairs_overlap = file_lines
        .iter()
        .filter_map(|l| shared(pairs_from_line(l)))
        .count();

    println!("{:?}", pairs_overlap);
}
