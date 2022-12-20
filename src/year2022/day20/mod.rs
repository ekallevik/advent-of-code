use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{Result};
use itertools::Itertools;
use crate::utils::get_input;

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
    let mut coordinates: Vec<Coordinate> = get_input(filename);

    mix_coordinates(&mut coordinates);

    //println!("{}", coordinates_to_string(&coordinates));

    let c = coordinates.clone();

    let offset = coordinates.iter().find_position(|v| match v {
        Coordinate::Encrypted(_) => unreachable!("asd"),
        Coordinate::Decrypted(value) => *value == 0
    }).unwrap().0;

    let first = c[(1000 + offset) % coordinates.len()].get_value();
    let second = c[(2000 + offset) % coordinates.len()].get_value();
    let third = c[(3000 + offset) % coordinates.len()].get_value();

    println!("{first} {second} {third}");

    let res = first + second + third;

    Ok(res.to_string())
}


pub fn solve_2(filename: &str) -> Result<String> {
    todo!()
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

    //println!("Moving {value} from {index} to {pos}");
    coordinates.insert(pos as usize, decrypted);
    //println!("{}\n", coordinates_to_string(coordinates));

    if pos <= index {
        IndexUpdate::Increment
    } else {
        IndexUpdate::Decrement
    }
}

fn get_new_index(index: usize, value: isize, size: isize) -> usize {

    let inx = index as isize;

    if value == 0 {
        index
    } else if value > 0 {
        ((inx + value) % size) as usize
    } else {
        let mut current = inx + value;

        while current < 0 {
            current += size;
        };

        current as usize
    }

}

enum IndexUpdate {
    Increment,
    Decrement,
}

fn coordinates_to_string(coordinates: &[Coordinate]) -> String {
    coordinates.iter().join(", ")
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