use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use strum_macros::{EnumIter};

pub mod solution;
pub(crate) mod string;

use anyhow::Result;
use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;
use serde::Deserialize;

pub type SolverFn = fn(&str) -> Result<String>;

#[derive(Debug, Deserialize, EnumIter)]
pub enum PuzzlePart {
    FirstTest,
    FirstReal,
    SecondTest,
    SecondReal,
}

impl Display for PuzzlePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzlePart::FirstTest => write!(f, "Part I - test"),
            PuzzlePart::FirstReal => write!(f, "Part I - real"),
            PuzzlePart::SecondTest => write!(f, "Part II - test"),
            PuzzlePart::SecondReal => write!(f, "Part II - real"),
        }
    }
}


impl ToSql for PuzzlePart {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

pub fn get_input_string(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("file not found!")
}

pub fn get_input<T: FromStr>(filename: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn get_partitioned_input(filename: &str) -> (String, String) {
    let input = std::fs::read_to_string(filename).expect("file not found!");
    let (first, second) = input.split_once("\n\n").unwrap();
    (first.to_string(), second.to_string())
}

pub fn get_comma_seperated_input<T: FromStr>(filename: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub fn get_input_array<T: FromStr>(filename: &str) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|y| y.to_string().parse().unwrap()).collect())
        .collect()
}

pub fn parse_3d_measurement(measurement: String) -> (u32, u32, u32) {
    let mut values = measurement
        .split('x')
        .map( |c| c.parse().unwrap())
        .collect::<Vec<_>>();

    (values.pop().unwrap(), values.pop().unwrap(), values.pop().unwrap())
}

pub fn breakpoint(message: &str) {
    let mut is_correct = String::new();
    println!("{}", message);
    std::io::stdin().read_line(&mut is_correct).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_input_array;

    #[test]
    fn test_get_input() {
        let result: Vec<String> = get_input("src/day03/test1.txt");

        let expected = vec!["00100", "11110"];

        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_input_array() {
        let result: Vec<Vec<u8>> = get_input_array("src/day03/test1.txt");

        let expected = vec![vec![0, 0, 1, 0, 0], vec![1, 1, 1, 1, 0]];

        assert_eq!(result, expected)
    }

    #[test]
    fn test() {
        assert_eq!(u8::from_str_radix("01", 2), Ok(1));
        assert_eq!(u8::from_str_radix("11", 2), Ok(3));
        assert_eq!(u8::from_str_radix("10", 2), Ok(2));
    }

}

