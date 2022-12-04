use std::str::FromStr;
use anyhow::{Result};
use crate::utils::get_input;

struct Pair {
    left: Range,
    right: Range,
}

impl Pair {
    fn is_redundant(&self) -> bool {
        self.left.overlaps(&self.right) || self.right.overlaps(&self.left)
    }

    fn intersects(&self) -> bool {
        self.left.intersects(&self.right) || self.right.intersects(&self.left)
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, right) = s.split_once(",").unwrap();
        Ok(
            Pair {
                left: left.parse().unwrap(),
                right: right.parse().unwrap()
            }
        )
    }
}

struct Range {
    from: usize,
    to: usize,
}

impl Range {

    fn overlaps(&self, other: &Self) -> bool {
        self.from <= other.from && other.to <= self.to
    }

    fn intersects(&self, other: &Self) -> bool {
        self.from <= other.from && other.from <= self.to
    }

}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (from, to) = s.split_once("-").unwrap();
        Ok(Range {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        })
    }
}



pub fn solve_1(filename: &str) -> Result<String> {
    let input: Vec<Pair> = get_input(filename);

    let score = input
        .iter()
        .filter(|pairs| pairs.is_redundant())
        .count();

    Ok(score.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input: Vec<Pair> = get_input(filename);

    let score = input
        .iter()
        .filter(|pairs| pairs.intersects())
        .count();

    Ok(score.to_string())
}
