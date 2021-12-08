mod utils;
mod day01;
mod day02;
mod domain;

use crate::utils::PuzzlePart;

fn main() {
    println!("Solving Advent of Code");
    println!("{}", day02::solve(PuzzlePart::Part2));
}