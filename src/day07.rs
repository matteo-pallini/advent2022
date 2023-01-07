use crate::utils::lines_from_file;

pub fn run() -> Option<u8> {
    let logs: Vec<String> = lines_from_file("src/day07.txt");
    let mut sizes_stack: Vec<i32> = vec![];
    let mut current_size: i32 = 0;
    let mut total_size: i32 = 0;
    for log in logs {
        current_size = if log.starts_with("$ cd ..") {
            if current_size <= 100000 {
                total_size += current_size
            }
            *sizes_stack.last_mut().unwrap() += current_size;
            current_size = sizes_stack.pop().unwrap();
            current_size
        } else if log.starts_with("$ cd") {
            sizes_stack.push(current_size.clone());
            0
        } else if let Ok(size) = log.split_whitespace().next().unwrap().parse::<i32>() {
            current_size + size
        } else {
            current_size
        };
    }
    println!("problem 7 - step total {}", total_size);
    None
}
