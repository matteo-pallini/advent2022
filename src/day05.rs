use std::{collections::HashMap, convert::TryInto};

use crate::utils::lines_from_file;

pub fn run() -> Option<u8> {
    let mut crates: HashMap<u8, Vec<&str>> = HashMap::from([
        (1, vec!["b", "l", "d", "t", "w", "c", "f", "m"]),
        (2, vec!["n", "b", "l"]),
        (3, vec!["j", "c", "h", "t", "l", "v"]),
        (4, vec!["s", "p", "j", "w"]),
        (5, vec!["z", "s", "c", "f", "t", "l", "r"]),
        (6, vec!["w", "d", "g", "b", "h", "n", "z"]),
        (7, vec!["f", "m", "s", "p", "v", "g", "c", "n"]),
        (8, vec!["w", "q", "r", "j", "f", "v", "c", "z"]),
        (9, vec!["r", "p", "m", "l", "h"]),
    ]);
    let mut crates_reverse = crates.clone();

    let crates_ids: Vec<[u8; 3]> = lines_from_file("src/day05.txt")
        .iter()
        .map(|line| {
            line.split_whitespace()
                .enumerate()
                .filter(|&(idx, _)| (idx == 1) || (idx == 3) || (idx == 5))
                .map(|(_, value)| value.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect();

    for &indexes in crates_ids.iter() {
        let (shift_size, origin_id, destination_id): (u8, u8, u8) =
            (indexes[0], indexes[1], indexes[2]);
        let start_point_reverse: usize =
            crates_reverse[&origin_id].len() - usize::try_from(shift_size).unwrap();
        let mut to_be_shifted_reverse: Vec<&str> = crates_reverse
            .get_mut(&origin_id)
            .unwrap()
            .drain(start_point_reverse..)
            .collect();
        to_be_shifted_reverse.reverse();
        crates_reverse
            .entry(destination_id)
            .and_modify(|vector| vector.append(&mut to_be_shifted_reverse));

        let start_point: usize = crates[&origin_id].len() - usize::try_from(shift_size).unwrap();
        let mut to_be_shifted: Vec<&str> = crates
            .get_mut(&origin_id)
            .unwrap()
            .drain(start_point..)
            .collect();
        crates
            .entry(destination_id)
            .and_modify(|vector| vector.append(&mut to_be_shifted));
    }
    let mut chars_reverse: String = String::new();
    let mut chars: String = String::new();
    for key in 1..10 {
        chars_reverse.push_str(*crates_reverse.get(&key).unwrap().last().unwrap());
        chars.push_str(*crates.get(&key).unwrap().last().unwrap());
    }
    println!("day5 - step 1 {}", chars_reverse);
    println!("day5 - step 2 {}", chars);
    None
}
