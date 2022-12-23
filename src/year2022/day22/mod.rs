use std::collections::HashMap;
use anyhow::{bail, Result};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use crate::utils::get_input;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position(isize, isize);


#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_neighbors(&self) -> [(isize, isize); 3] {
        match self {
            Direction::North => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::South => [(1, -1), (1, 0), (1, 1)],
            Direction::West => [(-1, -1), (0, -1), (1, -1)],
            Direction::East => [(-1, 1), (0, 1), (1, 1)],
        }
    }

    fn move_position(&self, position: &Position) -> Position {
        match self {
            Direction::North => Position(position.0 - 1, position.1),
            Direction::South => Position(position.0 + 1, position.1),
            Direction::West => Position(position.0, position.1 - 1),
            Direction::East => Position(position.0, position.1 + 1),
        }
    }
}

fn parse_grove(filename: &str) -> Vec<Position> {
    let lines: Vec<String> = get_input(filename);

    let mut positions = vec![];

    for (row, line) in lines.iter().enumerate() {
        for (col, elem) in line.chars().enumerate() {
            if elem == '#' {
                let pos = Position(row as isize, col as isize);
                positions.push(pos);
            }
        }
    }

    positions
}


pub fn solve_1(filename: &str) -> Result<String> {
    let mut grove = parse_grove(filename);

    let number_of_elves = grove.len();
    let mut directions = [Direction::North, Direction::South, Direction::West, Direction::East];
    let rounds = 10;

    print_grove(&grove);
    let bar = ProgressBar::new(rounds);
    bar.set_style(ProgressStyle::with_template("elapsed: [{elapsed_precise}] {bar:40.cyan/blue} {percent}")
        .unwrap());

    for round in 1..=rounds {
        let (updated_grove, _) = update_grove(&grove, &directions); // fixme: replace moves with updated-bool


        println!("\n\n== End of Round {round} ==");
        grove = updated_grove;
        print_grove(&grove);
        directions.rotate_left(1);
        bar.inc(1);
    }

    bar.finish();

    let empty_space = get_grove_size(&grove) as usize - number_of_elves;
    Ok(empty_space.to_string())
}


pub fn solve_2(filename: &str) -> Result<String> {
    let mut grove = parse_grove(filename);
    println!("Grove size {}", get_grove_size(&grove));
    print_grove(&grove);

    let mut directions = [Direction::North, Direction::South, Direction::West, Direction::East];

    let mut round = 1;
    loop {
        let (updated_grove, number_of_moves) = update_grove(&grove, &directions); // fixme: replace moves with updated-bool

        println!("\n== End of Round {round}: {} moves ==", number_of_moves);
        if number_of_moves == 0 {
            break
        } else {
            round += 1
        }

        grove = updated_grove;
        directions.rotate_left(1);
    }

    Ok(round.to_string())
}

fn is_movable_in_direction(grove: &[Position], elf: &Position, direction: &Direction) -> bool {

    for neighbor in direction.get_neighbors() {
        let neighbor_position = Position(elf.0 + neighbor.0, elf.1 + neighbor.1);
        if grove.contains(&neighbor_position) {
            return false;
        }
    }

    true
}

fn is_movable(grove: &[Position], elf: &Position) -> bool {

    let res = [
        is_movable_in_direction(grove, elf, &Direction::North),
        is_movable_in_direction(grove, elf, &Direction::South),
        is_movable_in_direction(grove, elf, &Direction::West),
        is_movable_in_direction(grove, elf, &Direction::East),
    ];

    let count = res.iter().filter(|&r| *r).count();

    count != 0 && count != 4
}

fn get_proposed_move(grove: &[Position], directions: &[Direction], elf: &Position) -> Position {
    for direction in directions {
        if is_movable_in_direction(grove, elf, direction) {
            return direction.move_position(elf);
        }
    }

    panic!("Found no moves for movable elf");
}


fn get_grove_rectangle(grove: &[Position]) -> (Position, Position) {
    let x_values = grove.iter().map(|pos| pos.0).collect_vec();
    let y_values = grove.iter().map(|pos| pos.1).collect_vec();

    let north_west = Position(x_values.iter().min().unwrap().clone(), y_values.iter().min().unwrap().clone());
    let south_east = Position(x_values.iter().max().unwrap().clone(), y_values.iter().max().unwrap().clone());

    (north_west, south_east)
} //fixme: more idiomatic

fn get_grove_size(grove: &[Position]) -> isize {
    let (north_west, south_east) = get_grove_rectangle(&grove);

    (south_east.0 - north_west.0 + 1) * (south_east.1 - north_west.1 + 1)
}

fn print_grove(grove: &[Position]) {
    let (north_west, south_east) = get_grove_rectangle(&grove);

    for x in north_west.0..=south_east.0 {
        for y in north_west.1..=south_east.1 {
            let pos = Position(x, y);
            if grove.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn update_grove(grove: &[Position], directions: &[Direction]) -> (Vec<Position>, usize) {

    let mut updated_grove = vec![];
    let mut number_of_moves = 0;

    let mut props: HashMap<Position, Vec<Position>> = HashMap::new(); //fixme: use references?

    for elf in grove {
        if is_movable(grove, elf) {
            let proposed_move = get_proposed_move(&grove, &directions, elf);

            props
                .entry(proposed_move)
                .and_modify(|vec| vec.push(elf.to_owned()))
                .or_insert(vec![elf.to_owned()]);
        } else {
            updated_grove.push(elf.to_owned());
        }
    }

    for (proposal, elves) in props {
        if elves.len() == 1 {
            updated_grove.push(proposal.to_owned().clone());
            number_of_moves += 1;
        } else if elves.len() > 1 {
            for elf in elves {
                updated_grove.push(elf.clone());
            }
        }
    };

    (updated_grove, number_of_moves)
}