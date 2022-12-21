use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use crate::utils::get_input;

enum SandMove {
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position(usize, usize);

impl Position {
    fn get_next(&self, sand_move: &SandMove) -> Position {
        match sand_move {
            SandMove::Down => Position(self.0, self.1 + 1),
            SandMove::Left => Position(self.0 - 1, self.1 + 1),
            SandMove::Right => Position(self.0 + 1, self.1 + 1),
        }
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x, y) = scan_fmt!(s, "{},{}", usize, usize).unwrap();
        Ok(Position(x, y))
    }
}

#[derive(Debug)]
struct Line(Position, Position);

impl Line {
    fn get_x_bounds(&self) -> (usize, usize) {
        if self.0.0 < self.1.0 {
            (self.0.0, self.1.0)
        } else {
            (self.1.0, self.0.0)
        }
    }

    fn get_y_bounds(&self) -> (usize, usize) {
        if self.0.1 < self.1.1 {
            (self.0.1, self.1.1)
        } else {
            (self.1.1, self.0.1)
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let line = Line(start.parse().unwrap(), end.parse().unwrap());
        Ok(line)
    }
}

fn parse_row(row: &str) -> Vec<Line> {
    let split = row.split(" -> ").collect_vec();

    split
        .windows(2)
        .map(
            |values|
            Line(values[0].parse().unwrap(), values[1].parse().unwrap())
        )
        .collect_vec()
}

fn parse_input(filename: &str) -> Vec<Line> {
    let input: Vec<String> = get_input(filename);

    input
        .iter()
        .flat_map(|s| parse_row(s))
        .collect()
}


fn get_grid_size(lines: &[Line]) -> (usize, usize, usize, usize) {
    let mut min_row = lines[0].0.0;
    let mut min_col = lines[0].0.1;
    let mut max_row = lines[0].0.0;
    let mut max_col = lines[0].0.1;

    for line in lines {
        let (curr_min_x, curr_max_x) = line.get_x_bounds();
        if curr_min_x < min_row {
            min_row = curr_min_x;
        }

        if curr_max_x > max_row {
            max_row = curr_max_x;
        }

        let (curr_min_y, curr_max_y) = line.get_y_bounds();
        if curr_min_y < min_col {
            min_col = curr_min_y;
        }

        if curr_max_y > max_col {
            max_col = curr_max_y
        }
    };


    (min_row, min_col, max_row, max_col)
}

enum Regolith {
    Air,
    Rock,
    Sand,
    Source,
    Void,
}

impl Display for Regolith {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Regolith::Air => write!(f, "."),
            Regolith::Rock => write!(f, "#"),
            Regolith::Sand => write!(f, "o"),
            Regolith::Source => write!(f, "+"),
            Regolith::Void => write!(f, "|"),
        }
    }
}

pub fn solve_1(filename: &str) -> Result<String> {
    let lines = parse_input(filename);

    let mut grid = RegolithMap::new(lines, Position(500, 0));
    let mut grains = 0;

    loop {
        let success = grid.add_sand_1();

        if success {
            grains += 1;
        } else {
            break;
        }
    }

    println!("{grid}");
    Ok(grains.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let lines = parse_input(filename);

    let mut source = Position(500, 0);
    let mut grid = RegolithMap::new(lines, source.clone());
    let mut grains = 1;

    while let Some(pos) = grid.add_sand(source.clone()) {
        source = pos;
        grains += 1;
    }

    println!("{grid}");
    Ok(grains.to_string())
}

struct RegolithMap {
    values: HashMap<Position, Regolith>,
    source: Position,
    lower_bound: usize,
    left_bound: usize,
    right_bound: usize,
}

impl RegolithMap {

    fn new(lines: Vec<Line>, source: Position) -> Self {
        let (left_bound, _, right_bound, lowest_rock) = get_grid_size(&lines);
        let lower_bound = lowest_rock + 2;

        let mut values: HashMap<Position, Regolith> = HashMap::new();
        values.insert(source.clone(), Regolith::Source);

        for line in lines {
            let (min_y, max_y) = line.get_y_bounds();
            let (min_x, max_x) = line.get_x_bounds();

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    values.insert(Position(x, y), Regolith::Rock);
                }
            }
        }

        for x in left_bound..=right_bound {
            values.insert(Position(x, lower_bound), Regolith::Rock);
        }

        RegolithMap {
            values,
            source,
            lower_bound,
            left_bound,
            right_bound,
        }
    }

    fn get_value(&self, pos: &Position) -> &Regolith {
        self.values.get(pos).unwrap_or(&Regolith::Air)
    }

    fn set(&mut self, pos: Position, value: Regolith) {
        self.values.insert(pos, value);
    }

    fn is_available_space(&self, pos: &Position) -> bool {
        match self.get_value(pos) {
            Regolith::Rock | Regolith::Sand => false,
            Regolith::Air | Regolith::Source | Regolith::Void => true
        }
    }

    fn is_void(&self, pos: &Position) -> bool {
        pos.1 >= self.lower_bound
    }

    fn add_sand_1(&mut self) -> bool {
        let mut current = self.source.clone();

        loop {
            let left = current.get_next(&SandMove::Left);
            let down = current.get_next(&SandMove::Down);
            let right = current.get_next(&SandMove::Right);

            if self.is_void(&down) {
                return false;
            } else if self.is_available_space(&down) {
                current = down;
            } else if self.is_void(&left) {
                return false;
            } else if self.is_available_space(&left) {
                current = left;
            } else if self.is_void(&right) {
                return false;
            } else if self.is_available_space(&right) {
                current = right;
            } else {
                match self.get_value(&current) {
                    Regolith::Air => {
                        self.set(current, Regolith::Sand);
                        return true;
                    }
                    Regolith::Rock => panic!("Inside rock somehow"),
                    Regolith::Sand => panic!("Inside sand somehow"),
                    Regolith::Source => return false,
                    Regolith::Void => return false,
                }
            }
        }
    }

    fn add_sand(&mut self, source: Position) -> Option<Position> {
        let mut current = source.clone();

        let mut previous = if source.1 == 0 {
            None
        } else {
            Some(Position(source.0, source.1-1))
        };

        loop {
            let left = current.get_next(&SandMove::Left);
            let down = current.get_next(&SandMove::Down);
            let right = current.get_next(&SandMove::Right);

            if down.1 == self.lower_bound {
                self.set(current, Regolith::Sand);
                return previous;
            } else if self.is_available_space(&down) {
                previous = Some(current.clone());
                current = down;
            } else if self.is_available_space(&left) {
                previous = Some(current.clone());
                current = left;
            } else if self.is_available_space(&right) {
                previous = Some(current.clone());
                current = right;
            } else {
                match self.get_value(&current) {
                    Regolith::Air | Regolith::Source => {
                        self.set(current, Regolith::Sand);
                        return previous;
                    }
                    Regolith::Rock => panic!("Inside rock somehow"),
                    Regolith::Sand | Regolith::Void => return None,
                }
            }
        }
    }
}


impl Display for RegolithMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.lower_bound {
            write!(f, "{y:2} ").expect("TODO: panic message");
            for x in self.left_bound..=self.right_bound {
                let value = self.get_value(&Position(x, y));
                write!(f, "{value}").expect("");
            }
            writeln!(f).expect("");
        }

        Ok(())
    }
}

