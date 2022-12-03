

use anyhow::{Result};

use crate::utils::get_input;

pub fn solve_1(filename: &str) -> Result<String> {
    let score: usize = get_input::<String>(filename)
        .iter()
        .map(|line| play_first_strategy(line))
        .sum();

    Ok(score.to_string())
}

fn play_first_strategy(line: &str) -> usize {
    match line {
        "A X" => 3+1,
        "A Y" => 6+2,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 3+2,
        "B Z" => 6+3,
        "C X" => 6+1,
        "C Y" => 2,
        "C Z" => 3+3,
        _ => panic!("Should not happen")
    }
}

pub fn solve_2(filename: &str) -> Result<String> {
    let score: usize = get_input::<String>(filename)
    .iter()
    .map(|line| play_second_strategy(line))
    .sum();

    Ok(score.to_string())
}

fn play_second_strategy(line: &str) -> usize {
    match line {
        "A X" => 3,
        "A Y" => 3+1,
        "A Z" => 6+2,
        "B X" => 1,
        "B Y" => 3+2,
        "B Z" => 6+3,
        "C X" => 2,
        "C Y" => 3+3,
        "C Z" => 6+1,
        _ => panic!("Should not happen")
    }
}
