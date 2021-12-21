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

pub fn prime_factor(number: i32) -> Vec<i32> {
    let mut n = number;
    let mut primes = vec![];
    let mut candidate = 2;

    while n > 1 {
        while n % candidate == 0 {
            primes.push(candidate);
            n /= candidate
        }
        candidate += 1;
    }

    primes
}

pub fn factor(number: i32) -> Vec<i32> {
    (1..=number)
        .into_iter()
        .filter(|&x| number % x == 0)
        .collect::<Vec<i32>>()
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

    #[test]
    fn test_factor() {
        assert_eq!(factor(20), vec![1, 2, 4, 5, 10, 20]);
    }
}
