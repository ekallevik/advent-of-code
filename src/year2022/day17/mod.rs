use std::collections::HashSet;
use std::str::FromStr;
use anyhow::{bail, Result};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use crate::utils::{get_input, get_input_string};

#[derive(Debug, Eq, PartialEq)]
enum Rock {
    Horizontal,
    Plus,
    Angle,
    Vertical,
    Block,
}

type Position = (usize, usize);

impl Rock {
    fn get_positions(&self) -> Vec<Position> {
        match self {
            Rock::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => vec![(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
            Rock::Angle => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Block => vec![(0, 0), (1, 0), (0, 1), (1, 1)]
        }
    }
}

#[derive(Debug)]
enum Push {
    Left,
    Right,
}

impl FromStr for Push {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "<" {
            Ok(Push::Left)
        } else if s == ">" {
            Ok(Push::Right)
        } else {
            Err("Push not supported".to_string())
        }
    }
}

pub fn solve_1(filename: &str) -> Result<String> {
    let mut input = get_input_string(filename);
    let mut pushes = input.chars().map(|c| c.to_string().parse().unwrap()).collect_vec();

    let width = 7;
    let spawn_width = 2;
    let height_delta = 4;

    let mut rocks = [
        Rock::Horizontal,
        Rock::Plus,
        Rock::Angle,
        Rock::Vertical,
        Rock::Block
    ];

    let rounds = 2022;
    let mut height = 0;

    let mut grid: HashSet<Position> = (0..width).into_iter().map(|w| (w, 0)).collect();

    let bar = ProgressBar::new(rounds);
    bar.set_style(ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap());

    for _ in 1..=rounds {
        let mut rock_height = height + height_delta;
        let mut rock_width = spawn_width;

        let rock = &rocks[0];

        loop {
            let absolute_positions = rock
                .get_positions()
                .iter()
                .map(|relative| (relative.0 + rock_width, relative.1 + rock_height))
                .collect_vec();
            let push = &pushes[0];

            let can_push = absolute_positions
                .iter()
                .map(|p|
                    match push {
                        Push::Left => (p.0 - 1, p.1),
                        Push::Right => (p.0 + 1, p.1),
                    }
                )
                .all(|p| p.0 < width && !grid.contains(&p));

            let pushed_positions = if can_push {
                match push {
                    Push::Left => rock_width -= 1,
                    Push::Right => rock_width += 1,
                }

                let mut res = vec![];
                for mut position in absolute_positions {
                    let pushed = match push {
                        Push::Left => (position.0 - 1, position.1),
                        Push::Right => (position.0 + 1, position.1),
                    };
                    res.push(pushed);
                }
                res
            } else {
                absolute_positions
            };

            pushes.rotate_left(1);

            let cannot_fall = pushed_positions
                .iter()
                .map(|p| (p.0, p.1 - 1))
                .any(|p| grid.contains(&p));

            if cannot_fall {
                for position in pushed_positions {
                    grid.insert(position);
                }

                let highest = grid
                    .iter()
                    .max_by(|x, y| x.1.cmp(&y.1))
                    .unwrap();

                height = highest.1;

                break;
            } else {
                rock_height -= 1;
            }
        }

        bar.inc(1);
        bar.set_message(format!("Height: {}", height));
        rocks.rotate_left(1);
    }

    bar.finish();

    Ok(height.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let mut input = get_input_string(filename);
    let mut pushes = input.chars().map(|c| c.to_string().parse().unwrap()).collect_vec();

    let width = 7;
    let spawn_width = 2;
    let height_delta = 4;

    let mut rocks = [
        Rock::Horizontal,
        Rock::Plus,
        Rock::Angle,
        Rock::Vertical,
        Rock::Block
    ];

    let rounds: isize = 1000000000000;
    let mut height = 0;
    let checkpoint = pushes.len();

    let mut grid: HashSet<Position> = (0..width).into_iter().map(|w| (w, 0)).collect();

    let bar = ProgressBar::new(rounds as u64);
    bar.set_style(ProgressStyle::with_template("elapsed: [{elapsed_precise}] eta: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap());

    println!("Checkpoint: {}, pushes: {}, rocks: {}", checkpoint, pushes.len(), rocks.len());

    let mut increases = vec![];
    let mut round = 0;
    let mut first_cycle_round = None;
    let mut cycle_size = None;
    let mut number_of_rocks_in_cycle = 0;

    loop {
        let mut rock_height = height + height_delta;
        let mut rock_width = spawn_width;

        let rock = &rocks[0];

        loop {
            let absolute_positions = rock
                .get_positions()
                .iter()
                .map(|relative| (relative.0 + rock_width, relative.1 + rock_height))
                .collect_vec();
            let push = &pushes[0];

            let can_push = absolute_positions
                .iter()
                .map(|p|
                    match push {
                        Push::Left => (p.0 - 1, p.1),
                        Push::Right => (p.0 + 1, p.1),
                    }
                )
                .all(|p| p.0 < width && !grid.contains(&p));

            let pushed_positions = if can_push {
                match push {
                    Push::Left => rock_width -= 1,
                    Push::Right => rock_width += 1,
                }

                let mut res = vec![];
                for mut position in absolute_positions {
                    let pushed = match push {
                        Push::Left => (position.0 - 1, position.1),
                        Push::Right => (position.0 + 1, position.1),
                    };
                    res.push(pushed);
                }
                res
            } else {
                absolute_positions
            };

            pushes.rotate_left(1);

            let cannot_fall = pushed_positions
                .iter()
                .map(|p| (p.0, p.1 - 1))
                .any(|p| grid.contains(&p));

            if cannot_fall {
                for position in pushed_positions {
                    grid.insert(position);
                }

                let highest = grid
                    .iter()
                    .max_by(|x, y| x.1.cmp(&y.1))
                    .unwrap();

                let height_delta = highest.1 - height;
                increases.push(height_delta);
                height = highest.1;

                break;
            } else {
                rock_height -= 1;
            }
        }

        bar.inc(1);
        bar.set_message(format!("Round: {}, Height: {}", round, height));

        round += 1;

        if let Some(first_round) = first_cycle_round {

            let size = cycle_size.unwrap() as isize;

            if round == first_round + size {
                //print_grid(&grid, width, height, &vec![]);

                println!("round: {}, height: {}, rocks: {}", round, height, number_of_rocks_in_cycle);
                let remaining_cycles = (rounds - round) / number_of_rocks_in_cycle;
                let pattern_height = remaining_cycles * size;
                println!("cycle size: {}, remaining cycles: {}, pattern_height: {}", size, remaining_cycles, pattern_height);

                break;
            }
        } else if round > 10 {
            if let Some(cycle_size_1) = is_offset_cyclic(&grid, height, width) {
                println!("round: {}, height: {}", round, height);

                cycle_size = Some(cycle_size_1);
                first_cycle_round = Some(round);

            }
        }

        if first_cycle_round.is_some() {
            number_of_rocks_in_cycle += 1;
        }

        if round == rounds {
            break;
        }

        rocks.rotate_left(1);
    }

    bar.finish();

    Ok(height.to_string())
}


fn print_grid(grid: &HashSet<Position>, width: usize, height: usize, rock: &[Position]) {
    for h in (0..=height).rev() {
        if h == 0 {
            print!("+");
        } else {
            print!("|");
        }

        for w in 0..width {
            if h == 0 {
                print!("-");
            } else if rock.contains(&(w, h)) {
                print!("@");
            } else if grid.contains(&(w, h)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        if h == 0 {
            println!("+");
        } else {
            println!("|");
        }
    }
}

fn is_offset_cyclic(grid: &HashSet<Position>, height: usize, width: usize) -> Option<usize> {
    if height % 2 != 0 {
        return None;
    }

    for segment_size in 20..100 {
        if is_cyclic_segment(grid, height, width, segment_size) {

            return Some(segment_size);
        }
    }


    None
}

fn is_cyclic_segment(grid: &HashSet<Position>, height: usize, width: usize, cycle_size: usize) -> bool {
    for w in 0..width {
        for delta_h in 0..cycle_size {
            let a = grid.contains(&(w, height - delta_h));
            let b = grid.contains(&(w, height - cycle_size - delta_h));

            if a != b {
                return false;
            }
        }
    }

    true
}

fn is_cyclic(grid: &HashSet<Position>, height: usize, width: usize) -> bool {
    if height % 2 != 0 {
        return false;
    }

    let midpoint = height / 2;
    //println!("mid: {midpoint}");

    for w in 0..width {
        for delta_h in 0..midpoint {

            //println!("w: {w}, delta_h: {delta_h}");
            let a = grid.contains(&(w, delta_h));
            let b = grid.contains(&(w, midpoint + delta_h));


            if a != b {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cyclic() {
        let mut grid = HashSet::new();
        grid.insert((2, 0));
        grid.insert((3, 0));
        grid.insert((4, 0));

        grid.insert((2, 1));
        grid.insert((3, 1));
        grid.insert((4, 1));

        assert!(is_cyclic(&grid, 2, 2))
    }
}
