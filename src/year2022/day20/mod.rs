use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{Result};
use itertools::Itertools;
use crate::utils::get_input;

#[derive(Debug)]
struct GrooveCoordinate {
    id: usize,
    value: isize,
}

impl Display for GrooveCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:12}", self.value)
    }
}

#[derive(Debug, Clone, Copy)]
enum Coordinate {
    Encrypted(isize),
    Decrypted(isize),
}

impl Coordinate {
    fn get_value(&self) -> isize {
        match self {
            Coordinate::Encrypted(v) => v.clone(),
            Coordinate::Decrypted(v) => v.clone()
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Coordinate::Encrypted(value) => write!(f, "e:{}", value),
            Coordinate::Decrypted(value) => write!(f, "d:{}", value),
        }
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Coordinate::Encrypted(s.parse().unwrap()))
    }
}

pub fn solve_1(filename: &str) -> Result<String> {
    let values: Vec<isize> = get_input(filename);

    let mut coordinates = values
        .into_iter()
        .enumerate()
        .map(|(id, value)| GrooveCoordinate { id, value })
        .collect_vec();

    mix_grove_coordinates(&mut coordinates);

    let res = get_groove_coordinates_2(&coordinates);

    Ok(res.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let values: Vec<isize> = get_input(filename);

    const ENCRYPTION_KEY: isize = 811589153;

    let mut coordinates = values
        .into_iter()
        .enumerate()
        .map(|(id, value)| GrooveCoordinate { id, value: value * ENCRYPTION_KEY })
        .collect_vec();

    for _ in 0..10 {
        mix_grove_coordinates(&mut coordinates);
    }

    let res = get_groove_coordinates_2(&coordinates);

    Ok(res.to_string())
}

fn mix_grove_coordinates(coordinates: &mut Vec<GrooveCoordinate>) {
    let size = coordinates.len();
    for i in 0..size {
        let (c_i, _) = coordinates
            .iter()
            .find_position(|c| c.id == i)
            .unwrap();

        let coordinate = coordinates.remove(c_i);
        let pos = get_new_index(c_i, coordinate.value, (size-1) as isize);

        coordinates.insert(pos, coordinate);
    }
    println!("\n")
}

fn mix_coordinates(coordinates: &mut Vec<Coordinate>) {
    let mut index = 0;

    loop {
        let coordinate = coordinates.remove(index);

        let update = match coordinate {
            Coordinate::Encrypted(value) => {
                decrypt_value(value.to_owned(), index, coordinates)
            }
            Coordinate::Decrypted(_) => {
                coordinates.insert(index, coordinate);
                IndexUpdate::Increment
            }
        };

        match update {
            IndexUpdate::Increment => index += 1,
            IndexUpdate::Decrement => {}
        }

        let decrypted = coordinates.iter().all(|c| match c {
            Coordinate::Encrypted(_) => false,
            Coordinate::Decrypted(_) => true
        });

        if decrypted {
            break;
        }
    }
}

fn decrypt_value(value: isize, index: usize, coordinates: &mut Vec<Coordinate>) -> IndexUpdate {
    let decrypted = Coordinate::Decrypted(value);
    let pos = get_new_index(index, value, coordinates.len() as isize);

    coordinates.insert(pos as usize, decrypted);

    if pos <= index {
        IndexUpdate::Increment
    } else {
        IndexUpdate::Decrement
    }
}

fn get_new_index(index: usize, value: isize, size: isize) -> usize {
    let mut new_index = (index as isize + value) % size;
    if new_index < 0 {
        new_index = size + new_index
    };

    new_index as usize
}

enum IndexUpdate {
    Increment,
    Decrement,
}

fn get_groove_coordinates(coordinates: &[Coordinate]) -> isize {
    let offset = coordinates.iter().find_position(|v| match v {
        Coordinate::Encrypted(_) => unreachable!("asd"),
        Coordinate::Decrypted(value) => *value == 0
    }).unwrap().0;

    let size = coordinates.len();
    let first = coordinates[(1000 + offset) % size].get_value();
    let second = coordinates[(2000 + offset) % size].get_value();
    let third = coordinates[(3000 + offset) % size].get_value();

    first + second + third
}

fn get_groove_coordinates_2(coordinates: &[GrooveCoordinate]) -> isize {
    let (offset, _) = coordinates.iter().find_position(|v| v.value == 0).unwrap();

    let size = coordinates.len();
    let first = coordinates[(1000 + offset) % size].value;
    let second = coordinates[(2000 + offset) % size].value;
    let third = coordinates[(3000 + offset) % size].value;


    first + second + third
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let new = get_new_index(4, 4, 12);

        assert_eq!(new, 8)
    }

    #[test]
    fn test_forward_8k() {
        let new = get_new_index(0, 811589153, 7);

        assert_eq!(new, 6)
    }

    #[test]
    fn test_forward_with_overflow() {
        let new = get_new_index(4, 4, 6);

        assert_eq!(new, 2)
    }

    #[test]
    fn test_backward() {
        let new = get_new_index(4, -2, 6);

        assert_eq!(new, 2)
    }

    #[test]
    fn test_backward_with_overflow() {
        let new = get_new_index(4, -6, 8);

        assert_eq!(new, 6)
    }

    #[test]
    fn test_backward_with_large_overflow() {
        let new = get_new_index(4, -10, 8);

        assert_eq!(new, 2)
    }

    #[test]
    fn test_backward_with_large_overflow_2() {
        let new = get_new_index(2, -10, 4);

        assert_eq!(new, 0)
    }
}