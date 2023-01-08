use std::cmp;

use crate::utils::lines_from_file;

pub fn run() -> Option<u8> {
    let forest: Vec<Vec<u8>> = lines_from_file("data/day08.txt")
        .iter()
        .map(|row| row.chars().map(|v| v.to_digit(10).unwrap() as u8).collect())
        .collect();
    let row_length: usize = forest.first().unwrap().len();
    let mut forest_min_horiz: Vec<Vec<u8>> = vec![vec![0; row_length]; forest.len()];

    for r_idx in 0..forest.len() {
        for c_idx in 0..row_length {
            if (c_idx == 0) | (r_idx == 0) | (c_idx == row_length - 1) | (r_idx == forest.len() - 1)
            {
                forest_min_horiz[r_idx][c_idx] = 0;
            } else {
                let inverted_c_idx = row_length - 1 - c_idx;
                let inverted_r_idx = forest.len() - 1 - r_idx;
                forest_min_horiz[r_idx][c_idx] =
                    get_min_value_for_point(r_idx, c_idx, &forest, &forest_min_horiz);
                forest_min_horiz[r_idx][inverted_c_idx] =
                    get_min_value_for_point(r_idx, inverted_c_idx, &forest, &forest_min_horiz);
                forest_min_horiz[inverted_r_idx][c_idx] =
                    get_min_value_for_point(inverted_r_idx, c_idx, &forest, &forest_min_horiz);
                forest_min_horiz[inverted_r_idx][inverted_c_idx] = get_min_value_for_point(
                    inverted_r_idx,
                    inverted_c_idx,
                    &forest,
                    &forest_min_horiz,
                );
            }
        }
    }

    let mut counter = 0;
    for r_idx in 0..forest.len() {
        for c_idx in 0..row_length {
            println!(
                "{} {} {} {}",
                r_idx, c_idx, forest[r_idx][c_idx], forest_min_horiz[r_idx][c_idx]
            );
            if forest[r_idx][c_idx] > forest_min_horiz[r_idx][c_idx] {
                counter += 1
            }
        }
        println!("");
    }
    println!("day 8 - step 1: {}", counter);
    None
}

fn get_min_value_for_point(
    r_idx: usize,
    c_idx: usize,
    forest: &Vec<Vec<u8>>,
    forest_min: &Vec<Vec<u8>>,
) -> u8 {
    cmp::max(
        *vec![
            forest[r_idx - 1][c_idx],
            forest[r_idx][c_idx - 1],
            forest[r_idx + 1][c_idx],
            forest[r_idx][c_idx + 1],
        ]
        .iter()
        .min()
        .unwrap(),
        forest_min[r_idx][c_idx],
    )
}
