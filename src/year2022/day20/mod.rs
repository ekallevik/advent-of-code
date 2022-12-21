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

pub fn solve_1(filename: &str) -> Result<String> {
    let values: Vec<isize> = get_input(filename);

    let mut coordinates = values
        .into_iter()
        .enumerate()
        .map(|(id, value)| GrooveCoordinate { id, value })
        .collect_vec();

    mix_grove_coordinates(&mut coordinates);

    let res = get_groove_coordinates(&coordinates);
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

    let res = get_groove_coordinates(&coordinates);
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
}

fn get_new_index(index: usize, value: isize, size: isize) -> usize {
    let mut new_index = (index as isize + value) % size;
    if new_index < 0 {
        new_index = size + new_index
    };

    new_index as usize
}

fn get_groove_coordinates(coordinates: &[GrooveCoordinate]) -> isize {
    let (offset, _) = coordinates.iter().find_position(|v| v.value == 0).unwrap();

    let size = coordinates.len();
    let first = coordinates[(1000 + offset) % size].value;
    let second = coordinates[(2000 + offset) % size].value;
    let third = coordinates[(3000 + offset) % size].value;

    first + second + third
}
