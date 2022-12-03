use anyhow::{anyhow, Result};
use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> Result<String> {
    let directions = get_input_string(filename);

    let count = directions
        .chars()
        .fold(
            0,
            |acc, current|
                match current {
                    '(' => acc + 1,
                    ')' => acc - 1,
                    _ => panic!("Invalid input for direction")
                }
        )
        ;

    Ok(count.to_string())
}


pub fn solve_2(filename: &str) -> Result<String> {
    let directions = get_input_string(filename);

    println!("{}", directions.len());

    let mut floor = 0;

    for (index, char) in directions.chars().enumerate() {
        floor = match char {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => panic!("Invalid input for direction")
        };

        if floor == -1 {
            return Ok((index + 1).to_string())
        }


    }

    Err(anyhow!("Could not find an answer"))
}