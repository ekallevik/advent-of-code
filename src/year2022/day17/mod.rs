use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use anyhow::{bail, Result};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use crate::utils::{get_input, get_input_string};

#[derive(Debug)]
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
    bar.set_style(ProgressStyle::with_template("elapsed: [{elapsed_precise}] eta: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent}")
        .unwrap());

    for _ in 1..=rounds {
        let mut rock_height = height + height_delta;
        let mut rock_width = spawn_width;

        let rock = &rocks[0];

        //println!("\n\nRound {} - starting height = {}", round, height);

        loop {
            let absolute_positions = rock
                .get_positions()
                .iter()
                .map(|relative| (relative.0 + rock_width, relative.1 + rock_height))
                .collect_vec();
            let push = &pushes[0];

            /*
            println!("\nRock {:?} at pos: ({}, {})", rock, rock_width, rock_height);
            print_grid(&grid, width, rock_height+4, &absolute_positions);


             */

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
                //println!("Pushed one unit to the {:?}", push);
                res
            } else {
                //println!("Not able to push");
                absolute_positions
            };
            //print_grid(&grid, width, rock_height+4, &pushed_positions);


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
            bar.inc(1);
        }

        rocks.rotate_left(1);

        /*
        println!("\n\nRound {} - ending height = {}", round, height);
        print_grid(&grid, width, height, &vec![]);
        println!("\n\n");

         */
    }

    bar.finish();

    Ok(height.to_string())
}


pub fn solve_2(filename: &str) -> Result<String> {
    let round = 1;
    Ok(round.to_string())
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
