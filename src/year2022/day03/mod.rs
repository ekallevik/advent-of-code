use anyhow::{Result};
use itertools::Itertools;
use crate::utils::get_input;

pub fn solve_1(filename: &str) -> Result<String> {
    let input: Vec<String> = get_input(filename);

    let res: u32 = input
        .iter()
        .map(|line| find_realignment_priority(line))
        .sum();

    Ok(res.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input: Vec<String> = get_input(filename);

    let score: u32 = input
        .chunks(3)
        .map(find_badge_priority)
        .sum();

    Ok(score.to_string())
}

fn find_realignment_priority(rucksack: &str) -> u32 {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);

    let overlapping = left
        .chars()
        .filter(|c| right.contains(*c))
        .unique();

    overlapping
        .map(|c| get_char_priority(&c))
        .sum()
}

fn find_badge_priority(rucksack: &[String]) -> u32 {
    let chars = rucksack[0]
        .chars()
        .filter(|&c| rucksack[1].contains(c) && rucksack[2].contains(c))
        .collect::<Vec<char>>();

    get_char_priority(chars.first().unwrap())
}

fn get_char_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        *c as u32 - 38
    } else {
        *c as u32 - 96
    }
}
