use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut sum: i64 = 0;
    let mut max: i64 = 0;


    while stdin.lock().read_line(&mut line).unwrap() != 0{
        if line == "\n"{
            if &sum > &max{
                max = sum;
            }
            sum = 0;
        } else {
            let num = line.trim().parse().unwrap();
            sum += &num;
        }
        line.clear();
    }
    println!("{}", max)

}