use paris::{info};


use crate::utils::{get_input_array};

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

impl Octopus {
    fn step(mut self) -> Self {
        self.flashed = false;
        self.increase()
    }

    fn increase(mut self) -> Self {
        if self.energy == 9 {
            self.energy = 0;
            self.flashed = true;
        } else if !self.flashed {
            self.energy += 1;
        }

        self
    }

    fn flash(mut self) {
        self.flashed = true;
        self.energy = 0;
    }
}

type Cell = (u8, bool);

type Row = Vec<Cell>;

type Grid = Vec<Row>;

fn parse_input(filename: &String) -> Grid {
    let input = get_input_array::<u8>(filename);

    input
        .into_iter()
        .map(|row| row.into_iter().map(|c| (c, false)).collect::<Row>())
        .collect::<Grid>()
}

pub fn solve_1(filename: &String) -> String {
    let mut grid = parse_input(filename);
    let steps = 100;
    let mut flashes: u64 = 0;

    println!("Initial: \n {:?} \n", grid);

    for step in 1..=steps {
        grid = increase_energy(grid);

        loop {
            let no_of_flashes = flash(&mut grid);

            if no_of_flashes == 0 {
                break;
            } else {
                flashes += no_of_flashes;
            }
        }

        info!("Step  {} (flashes: {}):", step, flashes);
    }

    flashes.to_string()
}

fn increase_energy(grid: Grid) -> Grid {
    grid.into_iter()
        .map(|row| row.into_iter().map(|c| (c.0 + 1, false)).collect::<Row>())
        .collect::<Grid>()
}

fn flash(grid: &mut Grid) -> u64 {
    let mut no_of_flashes = 0;

    for i in 0..10 {
        for j in 0..10 {
            let current = &mut (*grid)[i as usize][j as usize];

            if !(*current).1 && (*current).0 > 9 {
                // flashing

                no_of_flashes += 1;
                (*current) = (0, true);

                let adjacents = get_adjacent(i, j, 10);

                for adjacent in adjacents.iter() {
                    let adjacent_cell = &mut (*grid)[adjacent.0 as usize][adjacent.1 as usize];
                    if !(*adjacent_cell).1 {
                        // increase energy
                        (*adjacent_cell) = ((*adjacent_cell).0 + 1, (*adjacent_cell).1)
                    }
                }
            }
        }
    }

    no_of_flashes
}

fn get_adjacent(i: i32, j: i32, max: i32) -> Vec<(i32, i32)> {
    let mut res = vec![];

    if i > 0 {
        res.push((i - 1, j));
    }

    if i > 0 && j > 0 {
        res.push((i - 1, j - 1));
    }

    if i > 0 && j < max - 1 {
        res.push((i - 1, j + 1));
    }

    if j > 0 {
        res.push((i, j - 1));
    }

    if j < max - 1 {
        res.push((i, j + 1));
    }

    if i < max - 1 {
        res.push((i + 1, j));
    }

    if i < max - 1 && j > 0 {
        res.push((i + 1, j - 1));
    }

    if i < max - 1 && j < max - 1 {
        res.push((i + 1, j + 1));
    }

    res
}

fn get_adjacent_with_overflow(i: i8, j: i8) -> Vec<(i8, i8)> {
    vec![
        (i - 1, j),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j),
        (i + 1, j - 1),
        (i + 1, j + 1),
    ]
}

fn flash_at(pos: (i8, i8), grid: &Grid) -> bool {
    if pos.0 < 0 || pos.1 < 0 || pos.0 > 9 || pos.1 > 9 {
        false
    } else {
        let c = grid[pos.0 as usize][pos.1 as usize];
        c.1
    }
}

pub fn solve_2(filename: &String) -> String {
    let mut grid = parse_input(filename);
    let mut step = 0;

    loop {
        step += 1;
        grid = increase_energy(grid);
        let mut flashes_in_step = 0;

        loop {
            let no_of_flashes = flash(&mut grid);

            flashes_in_step += no_of_flashes;

            if no_of_flashes == 0 {
                break;
            }
        }

        if flashes_in_step == 100 {
            return step.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_get_adjacent() {
        let result = get_adjacent(1, 1, 3);

        let expected = vec![
            (0, 1),
            (0, 0),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 1),
            (2, 0),
            (2, 2),
        ];

        assert_eq!(result, expected)
    }
}
