use std::collections::HashSet;
use anyhow::{bail, Result};
use scan_fmt::scan_fmt;
use crate::utils::{get_input, get_input_string};

type Position = (usize, usize, usize);

pub fn solve_1(filename: &str) -> Result<String> {
    let lines: Vec<String> = get_input(filename);

    let droplets: HashSet<Position> = lines
        .iter()
        .map(|line| {
            let a = scan_fmt!(line, "{},{},{}", usize, usize, usize).unwrap();
            a
        })
        .collect();

    let mut total_size = 0;

    for droplet in &droplets {
        let neighbors = get_neighbors(droplet);
        let number_of_neighbors = &neighbors
            .iter()
            .filter(|d| droplets.contains(d))
            .count();
        let free_sides = 6 - number_of_neighbors;
        total_size += free_sides;
    }

    Ok(total_size.to_string())
}

fn get_neighbors(droplet: &Position) -> Vec<Position> {
    let mut neighbors = vec![];

    if droplet.0 > 0 {
        neighbors.push((droplet.0 - 1, droplet.1, droplet.2));
    }

    if droplet.1 > 0 {
        neighbors.push((droplet.0, droplet.1 - 1, droplet.2));
    }

    if droplet.2 > 0 {
        neighbors.push((droplet.0, droplet.1, droplet.2 - 1));
    }

    neighbors.push((droplet.0 + 1, droplet.1, droplet.2));
    neighbors.push((droplet.0, droplet.1 + 1, droplet.2));
    neighbors.push((droplet.0, droplet.1, droplet.2 + 1));


    neighbors
}


pub fn solve_2(filename: &str) -> Result<String> {
    Ok("height".to_string())
}
