use crate::utils::lines_from_file;
use std::cmp;

struct MaxPoint {
    value: Option<u8>,
    idx: usize,
}

fn get_visibles_from_row<'a, I>(row: I) -> Vec<bool>
where
    I: Iterator<Item = &'a u8>,
{
    let max_point: MaxPoint = MaxPoint {
        value: None,
        idx: 0,
    };
    row.enumerate()
        .scan(max_point, |point, pair| {
            let visible: bool;
            let (_idx, value) = pair;
            (point.value, visible) = match point.value {
                Some(max_val) if max_val < *value => (Some(*value), true),
                Some(max_val) => (Some(max_val), false),
                None => (Some(*value), true),
            };
            Some(visible)
        })
        .collect::<Vec<bool>>()
}

fn build_forest_bools(forest: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut forest_bools: Vec<Vec<bool>> = Vec::new();
    for r_idx in 0..forest.len() {
        let visibles_l_t_r: Vec<bool> = get_visibles_from_row(forest[r_idx].iter());
        let mut visibles_r_t_l: Vec<bool> = get_visibles_from_row(forest[r_idx].iter().rev());
        visibles_r_t_l.reverse();
        let visibles_row: Vec<bool> = visibles_l_t_r
            .iter()
            .zip(visibles_r_t_l.iter())
            .map(|(l, r)| cmp::max(l, r).clone())
            .collect();
        forest_bools.push(visibles_row);
    }
    forest_bools
}

fn invert_forest<'a, T>(forest: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone + std::default::Default,
{
    let row_length: usize = forest.first().unwrap().len();
    let mut forest_inverted: Vec<Vec<T>> = vec![vec![Default::default(); forest.len()]; row_length];
    for r_idx in 0..forest.len() {
        for c_idx in 0..row_length {
            forest_inverted[c_idx][r_idx] = forest[r_idx][c_idx].clone()
        }
    }
    forest_inverted
}

pub fn run() -> Option<u8> {
    let forest: Vec<Vec<u8>> = lines_from_file("data/day08.txt")
        .iter()
        .map(|row| row.chars().map(|v| v.to_digit(10).unwrap() as u8).collect())
        .collect();
    let forest_bools_rows = build_forest_bools(&forest);
    let inverted_forest = invert_forest(&forest);
    let inverted_forest_bools = build_forest_bools(&inverted_forest);
    let forest_bools_cols = invert_forest(&inverted_forest_bools);
    let forest_bools: Vec<Vec<bool>> = forest_bools_rows
        .iter()
        .zip(forest_bools_cols.iter())
        .map(|(row, col)| {
            row.iter()
                .zip(col.iter())
                .map(|(r_val, c_val)| cmp::max(r_val, c_val).clone())
                .collect::<Vec<bool>>()
        })
        .collect();
    let visible_trees: u32 = forest_bools.iter().fold(0, |acc, row| {
        acc + row.iter().map(|val| *val as u32).sum::<u32>()
    });
    println!("day 8 alt - step 1: {}", visible_trees);

    None
}
