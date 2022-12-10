use min_max_heap::MinMaxHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Option<u8> {
    let mut current_value: u32 = 0;
    let mut heap = MinMaxHeap::with_capacity(3);
    let buffered = BufReader::new(File::open("src/day01.txt").ok()?);
    for line in buffered.lines() {
        if line.as_ref().unwrap().is_empty() {
            if heap.len() <= 3 {
                heap.push(current_value)
            } else {
                heap.push_pop_min(current_value);
            }
            current_value = 0
        } else {
            current_value += line.expect("no line").parse::<u32>().unwrap()
        }
    }

    let mut max_value: u32 = heap.pop_max().unwrap_or(0);
    println!("day1, max value is {}", max_value);
    max_value += heap.pop_max().unwrap_or(0);
    max_value += heap.pop_max().unwrap_or(0);
    println!("day1, max 3 values are {}", max_value);
    None
}
