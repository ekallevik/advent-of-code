mod utils;
mod day01;

use crate::utils::read_all;
use crate::day01::solve_day01;
use crate::utils::PuzzlePart::Part2;

fn main() {
    println!("Solving Advent of Code");
    println!("{}", solve_day01(Part2));
}