use std::fmt::Debug;
use std::str::FromStr;

use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum PuzzlePart {
    FirstTest,
    FirstReal,
    SecondTest,
    SecondReal,
}

pub fn get_input<T: FromStr>(file_name: &String) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn get_input_array<T: FromStr>(file_name: &String) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|y| y.to_string().parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_input_array;

    #[test]
    fn test_get_input() {
        let result: Vec<String> = get_input(&"src/input03_test2.txt".to_string());

        let expected = vec!["00100", "11110"];

        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_input_array() {
        let result: Vec<Vec<u8>> = get_input_array(&"src/input03_test2.txt".to_string());

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
