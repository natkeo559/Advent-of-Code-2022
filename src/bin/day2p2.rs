use std::io::{self, BufRead};
use std::collections::HashMap;

fn score(lines: Vec<String>) -> i32{
    let score_map_a = "ABC".chars().enumerate().map(|(i, c)| (c, (i + 1) as i32)).collect::<HashMap<_, _>>();
    let score_map_x = "XYZ".chars().enumerate().map(|(i, c)| (c, (i*3) as i32)).collect::<HashMap<_, _>>();

    let a = lines.iter()
                        .map(|l| score_map_a.get(&l.chars().nth(0).unwrap()).unwrap().to_owned());

    let wins_x = lines.iter()
                        .map(|l| score_map_x.get(&l.chars().nth(1).unwrap()).unwrap().to_owned());

    let x = a.zip(wins_x.to_owned()).map(|(a, w)| match (a, w) {
        (1,0) => 3,
        (2,0) => 1,
        (3,0) => 2,
        (1,3) => 1,
        (2,3) => 2,
        (3,3) => 3,
        (1,6) => 2,
        (2,6) => 3,
        (3,6) => 1,
        (_,_) => panic!("idk")
    });

    x.sum::<i32>() + wins_x.sum::<i32>()
}

fn main(){
    let lines: Vec<String>  = io::stdin()
                                .lock()
                                .lines()
                                .map(|l| l.unwrap().split_whitespace().collect()).collect();

    println!("{}", score(lines));
}