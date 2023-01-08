use std::cmp;

use crate::utils::lines_from_file;

pub fn run() -> Option<u8> {
    let forest: Vec<Vec<u8>> = lines_from_file("data/day08.txt")
        .iter()
        .map(|row| row.chars().map(|v| v.to_digit(10).unwrap() as u8).collect())
        .collect();
    let row_length: usize = forest.first().unwrap().len();
    let forest_min_horiz = get_forest_min(&forest);
    let forest_min_vert = invert_forest(&get_forest_min(&invert_forest(&forest)));

    let mut counter = 0;
    for r_idx in 0..forest.len() {
        for c_idx in 0..row_length {
            if (forest[r_idx][c_idx]
                > cmp::min(
                    forest_min_horiz[r_idx][c_idx],
                    forest_min_vert[r_idx][c_idx],
                ))
                | (c_idx == 0)
                | (c_idx == row_length - 1)
                | (r_idx == 0)
                | (r_idx == forest.len() - 1)
            {
                counter += 1
            }
        }
    }
    println!("day 8 - step 1: {}", counter);
    None
}

fn get_forest_min(forest: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let row_length: usize = forest.first().unwrap().len();
    let mut forest_min: Vec<Vec<u8>> = vec![vec![9; row_length]; forest.len()];

    for r_idx in 0..forest.len() {
        let mut left_to_right: Vec<u8> = vec![0; row_length];
        let mut right_to_left: Vec<u8> = vec![0; row_length];

        for c_idx in 0..row_length {
            if (c_idx == 0) | (c_idx == row_length - 1) {
                forest_min[r_idx][c_idx] = 0;
            } else {
                let inverted_c_idx = row_length - 1 - c_idx;
                left_to_right[c_idx] = cmp::max(forest[r_idx][c_idx - 1], left_to_right[c_idx - 1]);
                right_to_left[inverted_c_idx] = cmp::max(
                    forest[r_idx][inverted_c_idx + 1],
                    right_to_left[inverted_c_idx + 1],
                );
            }
        }
        for idx in 0..row_length {
            forest_min[r_idx][idx] = cmp::min(left_to_right[idx], right_to_left[idx]);
        }
    }
    forest_min
}

fn invert_forest(forest: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let row_length: usize = forest.first().unwrap().len();
    let mut forest_inverted: Vec<Vec<u8>> = vec![vec![0; forest.len()]; row_length];
    for r_idx in 0..forest.len() {
        for c_idx in 0..row_length {
            forest_inverted[c_idx][r_idx] = forest[r_idx][c_idx]
        }
    }
    forest_inverted
}
