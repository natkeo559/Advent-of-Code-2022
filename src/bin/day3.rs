use std::collections::HashSet;
use std::fs::read_to_string;

trait Common {
    fn common(&self, b: &str) -> Option<char>;
}

trait Priority {
    fn priority(self) -> u32;
}

impl Common for str {
    fn common(&self, b: &str) -> Option<char> {
        let (shorter, longer) = if self.len() > b.len() {
            (b, self)
        } else {
            (self, b)
        };

        let s: HashSet<char> = shorter.chars().collect();
        longer.chars().filter(|c| s.contains(c)).next()
    }
}

impl Priority for char {
    fn priority(self) -> u32 {
        let r = (self as u32) - if self.is_uppercase() { 38 } else { 96 };

        r
    }
}

fn main() {
    let file_lines = read_to_string("./input/day3.txt")
        .expect("Could not find input file")
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>();

    let mut p = Vec::new();

    for line in file_lines {
        let first = &line[..line.len() / 2];
        let second = &line[line.len() / 2..line.len()];

        let c = first.common(second).unwrap().priority();

        p.push(c);
    }

    println!("{}", p.iter().sum::<u32>());
}
