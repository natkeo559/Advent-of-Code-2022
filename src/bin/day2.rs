use std::io::{self, BufRead};
use std::collections::HashMap;

fn score(lines: Vec<String>) -> i32{
    let score_map_a = "ABC".chars().enumerate().map(|(i, c)| (c, (i + 1) as i32)).collect::<HashMap<_, _>>();
    let score_map_x = "XYZ".chars().enumerate().map(|(i, c)| (c, (i + 1) as i32)).collect::<HashMap<_, _>>();

    let a = lines.iter()
                        .map(|l| score_map_a.get(&l.chars().nth(0).unwrap()).unwrap().to_owned());

    let x = lines.iter()
                        .map(|l| score_map_x.get(&l.chars().nth(1).unwrap()).unwrap().to_owned());

    let wins_a = a.zip(x.clone()).map(|(a,x)| match (a,x) {
        (1,3) => 6,
        (3,2) => 6,
        (2,1) => 6,
        (1,1) => 3,
        (2,2) => 3,
        (3,3) => 3,
        (3,1) => 0,
        (2,3) => 0,
        (1,2) => 0,
        (_,_) => panic!("Unknown move used.")

    });

    let wins_x = wins_a.map(|a| match a {
        0 => 6,
        3 => 3,
        6 => 0,
        _ => panic!("How??")
    });

    let score_x = wins_x.zip(x).map(|(a,b)| a+b).sum::<i32>();

    score_x
}

fn main(){
    let lines: Vec<String>  = io::stdin()
                                .lock()
                                .lines()
                                .map(|l| l.unwrap().split_whitespace().collect())
                                .collect();

    println!("{}", score(lines));
}