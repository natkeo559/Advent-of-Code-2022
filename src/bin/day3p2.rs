use std::collections::HashSet;
use std::fs::read_to_string;

trait Common {
    fn common(&self) -> Option<char>;
}

trait Priority {
    fn priority(self) -> u32;
}

impl Common for [String] {
    fn common(&self) -> Option<char> {
        let mut strings = self.to_owned();
        strings.sort();

        let s: HashSet<char> = strings[0].chars().collect();

        let t: HashSet<char> = strings[2].chars().filter(|c| s.contains(c)).collect();

        strings[1].chars().filter(|c| t.contains(c)).next()
    }
}

impl Priority for char {
    fn priority(self) -> u32 {
        let r = (self as u32) - if self.is_uppercase() { 38 } else { 96 };

        r
    }
}

fn main() {
    let file_lines = read_to_string("./input/day3p2.txt")
        .expect("Could not find input file")
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>();

    let sums = file_lines
        .chunks(3)
        .map(|l| l.common().unwrap().priority())
        .sum::<u32>();

    println!("{}", sums);
}
