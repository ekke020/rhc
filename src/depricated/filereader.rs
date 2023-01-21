use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_by_line(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    Ok(lines)
}
