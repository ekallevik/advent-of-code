use std::fmt::Debug;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
pub enum PuzzlePart {
    Part1,
    Part2,
}

pub fn get_input<T: FromStr>(file_name: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}