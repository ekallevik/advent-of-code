mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod domain;
mod utils;
mod day09;

use crate::utils::PuzzlePart;

#[allow(dead_code)]
#[allow(unused_imports)]
fn main() {
    println!("Solving Advent of Code");
    println!("{}", day09::solve(PuzzlePart::Part1));
}
