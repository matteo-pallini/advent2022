use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Option<u8> {
    let conversion_map = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let winners_map = HashMap::from([("X", "Z"), ("Y", "X"), ("Z", "Y")]);
    let losers_map: HashMap<&str, &str> = winners_map.iter().map(|(k, v)| (*v, *k)).collect();
    let choice_points = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let buffered = BufReader::new(File::open("src/day02.txt").expect("file not found"));
    let mut points1: u32 = 0;
    let mut points2: u32 = 0;
    for line in buffered.lines() {
        let elements: Vec<&str> = line.as_ref().unwrap().split(' ').collect();
        let first: &str = conversion_map.get(&elements[0])?;
        points1 += get_score_part1(first, elements[1], &winners_map, &choice_points);
        points2 += get_score_part2(
            first,
            elements[1],
            &losers_map,
            &winners_map,
            &choice_points,
        );
    }
    println!("day2 step 1 points {}", points1);
    println!("day2 step 2 points {}", points2);
    None
}

fn get_score_part1(
    played: &str,
    response: &str,
    winners_map: &HashMap<&str, &str>,
    choice_points: &HashMap<&str, u32>,
) -> u32 {
    let score: u32 = if *winners_map.get(response).unwrap() == played {
        6
    } else if winners_map.get(played).unwrap() == &response {
        0
    } else {
        3
    };
    score + choice_points.get(response).unwrap()
}

fn get_score_part2(
    played: &str,
    outcome: &str,
    losers_map: &HashMap<&str, &str>,
    winners_map: &HashMap<&str, &str>,
    choice_points: &HashMap<&str, u32>,
) -> u32 {
    let (score, response): (u32, &str) = match outcome {
        "X" => (0, winners_map.get(played).unwrap()),
        "Y" => (3, played),
        _ => (6, losers_map.get(played).unwrap()),
    };

    score + choice_points.get(response).unwrap()
}
