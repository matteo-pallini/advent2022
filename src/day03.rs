use crate::utils::lines_from_file;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run() -> Option<u8> {
    let lines: Vec<String> = lines_from_file("data/day03.txt");
    let (scores1, scores2): (Vec<u32>, Vec<u32>) = lines
        .chunks(3)
        .map(|chunk| (problem1(chunk), problem2(chunk)))
        .unzip();
    let score1: u32 = scores1.iter().sum();
    let score2: u32 = scores2.iter().sum();
    println!("day3 - step 1 {}", score1);
    println!("day3 - step 2 {}", score2);
    None
}

fn problem1(lines: &[String]) -> u32 {
    let mut score: u32 = 0;
    for line in lines.iter() {
        let mut seen = HashSet::new();
        let length: usize = line.chars().count();
        for (i, c) in line.chars().enumerate() {
            if i < length / 2 {
                seen.insert(c);
            } else if seen.contains(&c) {
                score += convert_char_to_score(Some(&c));
                break;
            }
        }
    }
    score
}

fn problem2(lines: &[String]) -> u32 {
    let first_set: HashSet<char> = lines[0].chars().collect();
    let shared_char: HashSet<char> = lines.iter().fold(first_set, |acc, line| {
        HashSet::from_iter(line.chars().filter(|e| acc.contains(e)))
    });
    convert_char_to_score(shared_char.iter().next())
}

#[inline]
fn convert_char_to_score(character: Option<&char>) -> u32 {
    match character {
        None => 0,
        Some(character) => {
            character.to_digit(36).unwrap() - 9 + character.is_uppercase() as u32 * 26
        }
    }
}
