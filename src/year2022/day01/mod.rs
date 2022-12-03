use anyhow::{Result};
use crate::utils::get_input_string;
use crate::domain::order::pop_max;

fn parse_food_for_troop(filename: &str) -> Vec<usize> {
    let food_per_elf = get_input_string(filename)
        .split("\n\n")
        .map(parse_food_for_elf)
        .collect::<Vec<_>>();

    food_per_elf
        .iter()
        .map(|personal| personal.iter().sum::<usize>())
        .collect::<Vec<_>>()
}

fn parse_food_for_elf(food: &str) -> Vec<usize> {
    food
        .split('\n')
        .map(|a| a.parse().unwrap())
        .collect()
}

pub fn solve_1(filename: &str) -> Result<String> {
    let mut food = parse_food_for_troop(filename);

    let result = pop_max(&mut food);
    Ok(result.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let mut food = parse_food_for_troop(filename);

    let result = pop_max(&mut food)
        + pop_max(&mut food)
        + pop_max(&mut food);

    Ok(result.to_string())
}

