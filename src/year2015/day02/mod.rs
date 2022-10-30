use std::cmp::max;
use crate::utils::{get_input, parse_3d_measurement};

pub fn solve_1(filename: &str) -> String {
    let measurements = get_input(filename)
        .into_iter()
        .map(parse_3d_measurement);

    let area: u32 = measurements
        .map(calculate_area)
        .sum();

    area.to_string()
}

pub fn solve_2(filename: &str) -> String {
    let measurements = get_input(filename)
        .into_iter()
        .map(parse_3d_measurement);

    let ribbon_length: u32 = measurements
        .map(|measurement| calculate_shortest_perimeter(measurement) + calculate_volume(measurement))
        .sum();

    ribbon_length.to_string()
}

fn calculate_area((x, y, z): (u32, u32, u32)) -> u32 {

    let area = 2*x*y+2*x*z+2*y*z;
    let smallest = x*y*z/max(max(x, y), z);
    area+smallest
}

fn calculate_volume((x, y, z): (u32, u32, u32)) -> u32 {
    x*y*z
}

fn calculate_shortest_perimeter((x, y, z): (u32, u32, u32)) -> u32 {
    2*(x+y+z-max(max(x, y), z))
}