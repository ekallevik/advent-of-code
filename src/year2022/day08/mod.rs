use anyhow::{Result};
use itertools::Itertools;
use crate::utils::{get_input};

fn parse_grid(filename: &str) -> Vec<Vec<u32>> {
    let input: Vec<String> = get_input(filename);

    let grid = input
        .iter()
        .map(|row| row.
            chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect_vec()
        )
        .collect_vec();
    grid
}

pub fn solve_1(filename: &str) -> Result<String> {
    let grid = parse_grid(filename);

    let size = grid[1].len();
    let mut visible = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                visible += 1;
                continue;
            }

            let vertical = get_vertical(&grid, size,j);

            let left_max = row[0..j].iter().max().unwrap_or(&row[0]);
            let right_max = row[j + 1..size].iter().max().unwrap_or(&row.last().unwrap());
            let top_max = *vertical[0..i].iter().max().unwrap_or(&vertical[0]);
            let down_max = *vertical[i + 1..size].iter().max().unwrap_or(&vertical.last().unwrap());

            if (left_max < value || value > right_max) || (top_max < value || value > down_max) {
                visible += 1;
            }
        }
    }

    Ok(visible.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let grid = parse_grid(filename);

    let size = grid[1].len();
    let mut scenic = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                continue;
            }

            let vertical = get_vertical(&grid, size,j);

            let mut left_scenic = 0;
            for k in row[0..j].iter().rev() {
                left_scenic += 1;
                if value <= k {
                    break;
                }
            }

            let mut top_scenic = 0;
            for k in vertical[0..i].iter().rev() {
                top_scenic += 1;
                if value <= k {
                    break;
                }
            }

            let mut right_scenic = 0;
            for k in &row[j + 1..size] {
                right_scenic += 1;
                if value <= k {
                    break;
                }
            }
            let mut down_scenic = 0;
            for k in &vertical[i + 1..size] {
                down_scenic += 1;
                if value <= k {
                    break;
                }
            }

            let score = left_scenic * right_scenic * top_scenic * down_scenic;
            if score >= scenic {
                scenic = score;
            }
        }
    }

    Ok(scenic.to_string())
}

fn get_vertical<T>(grid: &Vec<Vec<T>>, size: usize, column: usize) -> Vec<&T> {
    (0..size)
        .into_iter()
        .map(|row| &grid[row][column])
        .collect()
}
