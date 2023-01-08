use crate::utils::lines_from_file;

pub fn run() -> Option<u8> {
    let lines: Vec<String> = lines_from_file("data/day04.txt");
    let items_contained: Vec<&String> = lines
        .iter()
        .filter(|line| is_any_interval_contained(*line))
        .collect();
    println!("day4 - step 1 {}", items_contained.len());

    let items_without_overlaps: Vec<&String> = lines
        .iter()
        .filter(|line| is_any_interval_not_touching(*line))
        .collect();
    println!(
        "day4 - step 2 {}",
        lines.len() - items_without_overlaps.len()
    );
    None
}

fn is_any_interval_contained(line: &String) -> bool {
    let intervals = line.split(",").collect::<Vec<&str>>();
    let (first_low, first_up): (u8, u8) = get_bounds(intervals[0]);
    let (second_low, second_up): (u8, u8) = get_bounds(intervals[1]);
    (((first_low <= second_low) & (second_up <= first_up))
        | ((second_low <= first_low) & (first_up <= second_up))) as bool
}

fn is_any_interval_not_touching(line: &String) -> bool {
    let intervals = line.split(",").collect::<Vec<&str>>();
    let (first_low, first_up): (u8, u8) = get_bounds(intervals[0]);
    let (second_low, second_up): (u8, u8) = get_bounds(intervals[1]);
    ((first_up < second_low) | (second_up < first_low)) as bool
}

fn get_bounds(interval: &str) -> (u8, u8) {
    let bounds = interval.split("-").collect::<Vec<&str>>();
    (
        bounds[0].parse::<u8>().unwrap(),
        bounds[1].parse::<u8>().unwrap(),
    )
}
