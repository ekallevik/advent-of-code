use std::collections::HashSet;
use anyhow::{bail, Result};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use crate::utils::{get_input, get_input_string};

type Position = (isize, isize, isize);

fn parse_droplets(filename: &str) -> HashSet<Position> {
    let lines: Vec<String> = get_input(filename);

    let droplets: HashSet<Position> = lines
        .iter()
        .map(|line| {
            let a = scan_fmt!(line, "{},{},{}", isize, isize, isize).unwrap();
            a
        })
        .collect();
    droplets
}

pub fn solve_1(filename: &str) -> Result<String> {
    let droplets = parse_droplets(filename);

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

pub fn solve_2(filename: &str) -> Result<String> {
    let droplets = parse_droplets(filename);

    let min = -1;
    let max_x = droplets.iter().map(|d| d.0).max().unwrap()+1;
    let max_y = droplets.iter().map(|d| d.1).max().unwrap()+1;
    let max_z = droplets.iter().map(|d| d.2).max().unwrap()+1;

    let mut open = vec![
        (min, min, max_z),
        (min, max_y, min),
        (min, max_y, max_z),
        (max_x, min, min),
        (max_x, min, max_z),
        (max_x, max_y, min),
        (max_x, max_y, max_z),
    ];

    let mut total_surface = 0;
    let mut outside = vec![];

    while let Some(current) = open.pop() {

        let neighbors = get_neighbors(&current);

        let steam = neighbors
            .iter()
            .filter(|n| !droplets.contains(*n))
            .filter(|n| !open.contains(n))
            .filter(|n| n.0 <= max_x && n.1 <= max_y && n.2 <= max_z)
            .filter(|n| !outside.contains(*n))
            .filter(|n| n.0 >= min && n.1 >= min && n.2 >= min)
            .collect_vec();

        let drops = neighbors
            .iter()
            .filter(|n| droplets.contains(*n))
            .collect_vec();

        total_surface += drops.len();

        open.extend(steam);
        outside.push(current.clone());
    }

    Ok(total_surface.to_string())
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