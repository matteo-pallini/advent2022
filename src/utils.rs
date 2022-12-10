use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    // shamefully stolen from https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
    let vector: Result<Vec<String>> = BufReader::new(File::open(filename).expect("file not found"))
        .lines()
        .collect();
    vector.unwrap()
}
