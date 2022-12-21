use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::Result;
use crate::utils::{get_input};

#[derive(Debug)]
enum Move {
    Up(isize),
    Right(isize),
    Down(isize),
    Left(isize),
}

impl Move {
    fn steps(&self) -> &isize {
        match self {
            Move::Up(steps) => steps,
            Move::Right(steps) => steps,
            Move::Down(steps) => steps,
            Move::Left(steps) => steps,
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (direction, steps) = s.split_once(' ').unwrap();

        let steps = steps.parse().unwrap();

        let movement = match direction {
            "U" => Self::Up(steps),
            "R" => Self::Right(steps),
            "D" => Self::Down(steps),
            "L" => Self::Left(steps),
            _ => return Err(format!("Invalid direction: {direction}"))
        };

        Ok(movement)
    }
}


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Position(isize, isize);

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Position {
    fn increment(&mut self, movement: &Move) {
        match movement {
            Move::Up(_) => self.1 += 1,
            Move::Right(_) => self.0 += 1,
            Move::Down(_) => self.1 -= 1,
            Move::Left(_) => self.0 -= 1,
        }
    }

    fn catchup(&mut self, head: &Position) {
        let delta_x = head.0 - self.0;
        let delta_y = head.1 - self.1;

        if delta_x.abs() >= 1 && delta_y.abs() > 1 || delta_x.abs() > 1 && delta_y.abs() >= 1 {
            self.0 += delta_x / delta_x.abs();
            self.1 += delta_y / delta_y.abs();
        } else if delta_x.abs() >= 2 {
            self.0 += delta_x / delta_x.abs();
        } else if delta_y.abs() >= 2 {
            self.1 += delta_y / delta_y.abs();
        }
    }
}


pub fn solve_1(filename: &str) -> Result<String> {
    let movements: Vec<Move> = get_input(filename);

    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);

    let mut visited: HashSet<Position> = HashSet::new();

    for movement in movements {
        for _ in 0..*movement.steps() {
            head.increment(&movement);
            tail.catchup(&head);
            visited.insert(tail.clone());
        }
    }

    Ok(visited.len().to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let movements: Vec<Move> = get_input(filename);

    let mut ropes = vec![
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
        Position(0, 0),
    ];

    let mut visited: HashSet<Position> = HashSet::new();

    for movement in movements {
        for _ in 0..*movement.steps() {
            ropes[0].increment(&movement);

            for i in 1..ropes.len() {
                let head = &ropes[i - 1].clone();
                ropes[i].catchup(head);
            }

            visited.insert(ropes[9].clone());
        }
    }

    Ok(visited.len().to_string())
}
