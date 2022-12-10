use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Option<u8> {
    let mut score1: u32 = 0;
    let mut score2: u32 = 0;

    let buffered = BufReader::new(File::open("src/day03.txt").expect("file not found"));
    let previous_set: HashSet<char> = HashSet::new();
    for (line_i, line) in buffered.lines().enumerate() {
        let mut seen: HashSet<char>;
        let length: usize = line.as_ref().unwrap().chars().count();
        for (i, c) in line.as_ref().unwrap().chars().enumerate() {
            // problem 1
            if i < length / 2 {
                seen.insert(c);
            } else if seen.contains(&c) {
                score1 += convert_char_to_score(Some(&c));
                break;
            }
        }

        // problem 2
        let foo: String = previous_set.iter().collect();
        println!("\n prex state {} {}", line_i, foo);
        let previous_set: HashSet<char> = if line_i % 3 == 0 {
            let foo: String = previous_set.iter().collect();
            println!("first pre {} {}", line_i, foo);
            score2 = score2 + convert_char_to_score(previous_set.iter().nth(0));
            let previous_set: HashSet<char> = line.as_ref().unwrap().chars().collect();
            let foo: String = previous_set.iter().collect();
            println!("first post {} {}", line_i, foo);
            previous_set
        } else {
            let previous_set: HashSet<char> = previous_set;
            let foo: String = previous_set.iter().collect();
            println!("check {} {}", line_i, foo);
            let previous_set: HashSet<char> = line
                .as_ref()
                .unwrap()
                .chars()
                .filter(|c| previous_set.contains(c))
                .collect();
            previous_set
        };
        let foo: String = previous_set.iter().collect();
        println!("outside, {}", foo);
        // if line_i % 3 == 2 {
        //     let foo: String = previous_set.iter().collect();
        //     println!("{} {}", line_i, foo);
        //     //score2 = score2 + convert_char_to_score(previous_set.iter().nth(0).unwrap());
        //     previous_set.clear();
        // }
    }
    println!("day 3 step 1 score {}", score1);
    println!("day 3 step 2 score {}", score2);
    None
}

// fn problem2(line: Line, previous_set: HashSet<char>, score: u32) -> u32 {
//     if line_i % 3 == 0 {
//         let foo: String = previous_set.iter().collect();
//         println!("first pre {} {}", line_i, foo);
//         score2 = score2 + convert_char_to_score(previous_set.iter().nth(0));
//         let previous_set: HashSet<char> = line.as_ref().unwrap().chars().collect();
//         let foo: String = previous_set.iter().collect();
//         println!("first post {} {}", line_i, foo);
//     } else {
//         let foo: String = previous_set.iter().collect();
//         println!("check {} {}", line_i, foo);
//         let previous_set: HashSet<char> = line
//             .as_ref()
//             .unwrap()
//             .chars()
//             .filter(|c| previous_set.contains(c))
//             .collect();
//     }
// }

#[inline]
fn convert_char_to_score(character: Option<&char>) -> u32 {
    match character {
        None => 0,
        Some(character) => {
            character.to_digit(36).unwrap() - 9 + character.is_uppercase() as u32 * 26
        }
    }
}
