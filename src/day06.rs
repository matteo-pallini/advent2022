use std::collections::HashMap;

use crate::utils::lines_from_file;

pub fn run() -> Option<u8> {
    let message_chars: Vec<char> = lines_from_file("src/day06.txt")
        .first()
        .unwrap()
        .chars()
        .collect();
    let mut first_pointer: usize = 0;
    let mut second_pointer: usize = 0;
    let mut counter: HashMap<char, u8> = HashMap::with_capacity(4);
    while (second_pointer < message_chars.len()) && (counter.len() < 4) {
        if second_pointer - first_pointer > 3 {
            if *counter.get(&message_chars[first_pointer]).unwrap() == 1 as u8 {
                counter.remove(&message_chars[first_pointer]);
            } else {
                counter
                    .entry(message_chars[first_pointer])
                    .and_modify(|e| *e -= 1);
            }
            first_pointer += 1;
        } else {
            ();
        }
        *counter.entry(message_chars[second_pointer]).or_insert(0) += 1;
        second_pointer += 1;
    }
    println!("problem 6 - step 1 {}", second_pointer);
    None
}
