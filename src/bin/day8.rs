use ndarray::*;
use std::fs::read_to_string;

fn parse_input(input: String) -> Array2<u32> {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut arr = Array2::<u32>::default((grid.len(), grid.len()));

    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = grid[i][j];
        }
    }

    arr
}

fn process(v: ArrayBase<OwnedRepr<u32>, Dim<[usize; 1]>>) -> Vec<usize> {
    let mut tallest = 0;
    v.iter()
        .enumerate()
        .filter_map(|(index, item)| match item + 1 > tallest {
            true => {
                tallest = *item + 1;
                Some(index)
            }
            false => None,
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = read_to_string("./input/day8.txt").expect("Could not find input file");

    let mut grid = parse_input(input);

    let mut v_mask = Array2::<u32>::zeros((99, 99));

    for (row, v) in grid.axis_iter(Axis(0)).enumerate() {
        for index in process(v.to_owned()) {
            *v_mask.get_mut((row, index)).unwrap() = 1;
        }
    }

    grid.invert_axis(Axis(1));
    v_mask.invert_axis(Axis(1));

    for (row, v) in grid.axis_iter(Axis(0)).enumerate() {
        for index in process(v.to_owned()) {
            *v_mask.get_mut((row, index)).unwrap() = 1;
        }
    }

    for (col, v) in grid.axis_iter(Axis(1)).enumerate() {
        for index in process(v.to_owned()) {
            *v_mask.get_mut((index, col)).unwrap() = 1;
        }
    }

    grid.invert_axis(Axis(0));
    v_mask.invert_axis(Axis(0));

    for (col, v) in grid.axis_iter(Axis(1)).enumerate() {
        for index in process(v.to_owned()) {
            *v_mask.get_mut((index, col)).unwrap() = 1;
        }
    }

    println!("{:?}", v_mask.sum());
}
