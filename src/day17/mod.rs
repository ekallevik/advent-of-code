use crate::utils::factor;
use itertools::Itertools;

use std::ops::Range;

fn parse_input(filename: &str) -> (Range<i32>, Range<i32>) {
    let mut x_range = 0..0;
    let mut y_range = 0..0;

    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .for_each(|line| {
            let (_, _content) = line.split_once(": ").unwrap();
            let (x, y) = line.split_once(", ").unwrap();

            let (_, x) = x.split_once("=").unwrap();
            let (first, second) = x.split_once("..").unwrap();
            x_range = first.parse().unwrap()..(second.parse::<i32>().unwrap());

            let (_, y) = y.split_once("=").unwrap();
            let (first, second) = y.split_once("..").unwrap();
            y_range = first.parse().unwrap()..(second.parse::<i32>().unwrap());
        });

    (x_range, y_range)
}

pub fn solve_1(filename: &str) -> String {
    let (x_range, y_range) = parse_input(filename);

    // todo: check for off by one

    // todo: could be a set?
    let x_vel_candidates = (x_range.start..=x_range.end)
        .flat_map(factor)
        .unique()
        .sorted()
        .collect::<Vec<i32>>();

    println!("x_vel_candidates: \n{:?}", x_vel_candidates);

    /*
    let steps = x_vel_candidates
        .iter()
        .flat_map(|candidate|
        )


     */

    let _steps = 1..=x_range.end;

    //let y_values = vec![];
    for k in 1..=x_range.end {
        for target_y in y_range.start..=y_range.end {
            let sum: i32 = (0..k).sum();
            let y_velocity: f64 = (target_y as f64 - sum as f64) / k as f64;

            println!("k={}, y_vel={} for target={}", k, y_velocity, target_y);
        }
    }

    filename.to_string()
}

pub fn solve_2(filename: &str) -> String {
    let (_x_range, _y_range) = parse_input(filename);

    filename.to_string()
}
