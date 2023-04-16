use crate::utils::lines_from_file;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: i16,
    y: i16,
}

pub fn run() -> Option<u8> {
    let moves = lines_from_file("data/day09.txt");
    let mut visited: HashSet<Point> = HashSet::new();
    let mut head: Point = Point { x: 0, y: 0 };
    let mut tail: Point = Point { x: 0, y: 0 };
    let mut knots: Vec<Point> = vec![head];
    let mut prev_head: Point = tail;
    visited.insert(tail);

    for m in moves {
        let coord: Vec<&str> = m.split_whitespace().collect();
        let shifts: i16 = coord[1].parse::<i16>().unwrap();
        let direction: &str = coord[0];
        for _ in 0..shifts {
            if knots.len() > 8 {
                let _ = knots.pop().unwrap();
            }
            knots.insert(0, head.clone());
            prev_head = knots[0];
            //            println!("{}", format!("{prev_head:?}"));

            match direction {
                "R" => head.y += 1,
                "L" => head.y -= 1,
                "U" => head.x += 1,
                "D" => head.x -= 1,
                _ => println!("shouldn't happen"),
            }
            //          println!("{}", format!("{head:?} {tail:?} {prev_head:?}"));
            tail = if ((head.y - tail.y).abs() > 8) | ((head.x - tail.x).abs() > 8) {
                visited.insert(prev_head.clone());
                //            println!("{}", "1");
                prev_head
            } else {
                tail
            };
        }
    }
    println! {"day 9 - problem 1 {}", visited.len()};
    None
}
