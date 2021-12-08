use std::cmp::max;
use std::str::FromStr;

#[derive(Debug)]
pub struct Line {
    pub start: NaivePosition,
    pub end: NaivePosition,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs: Vec<&str> = s.split(" -> ").collect();

        Ok(Line {
            start: pairs[0].parse().unwrap(),
            end: pairs[0].parse().unwrap(),
        })
    }
}

impl Line {
    pub fn largest_point(&self) -> i64 {
        let max_x = max(self.start.x, self.end.x);
        let max_y = max(self.start.y, self.end.y);
        max(max_x, max_y)
    }

    pub fn get_start(&self) -> &NaivePosition {
        &self.start
    }
}

#[derive(Debug)]
pub enum SubmarineCommand {
    Up(i64),
    Down(i64),
    Forward(i64),
}

impl FromStr for SubmarineCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command: Vec<&str> = s.split_whitespace().collect();
        let distance: i64 = command[1].parse().unwrap();

        match command[0] {
            "forward" => Ok(SubmarineCommand::Forward(distance)),
            "up" => Ok(SubmarineCommand::Up(distance)),
            "down" => Ok(SubmarineCommand::Down(distance)),
            _ => Err(()),
        }
    }
}

impl SubmarineCommand {
    pub fn naive_apply_from(&self, point: NaivePosition) -> NaivePosition {
        match self {
            SubmarineCommand::Up(value) => point.up(value),
            SubmarineCommand::Down(value) => point.down(value),
            SubmarineCommand::Forward(value) => point.forward(value),
        }
    }

    pub fn apply_from(&self, position: Position) -> Position {
        match self {
            SubmarineCommand::Up(value) => position.up(value),
            SubmarineCommand::Down(value) => position.down(value),
            SubmarineCommand::Forward(value) => position.forward(value),
        }
    }
}

#[derive(Debug)]
pub struct NaivePosition {
    pub x: i64,
    pub y: i64,
}

impl FromStr for NaivePosition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i64> = s.split(',').map(|n| n.parse().unwrap()).collect();

        Ok(NaivePosition {
            x: numbers[0],
            y: numbers[1],
        })
    }
}

impl NaivePosition {
    fn up(&self, value: &i64) -> NaivePosition {
        NaivePosition {
            x: self.x,
            y: self.y + value,
        }
    }

    fn down(&self, value: &i64) -> NaivePosition {
        NaivePosition {
            x: self.x,
            y: self.y - value,
        }
    }

    fn forward(&self, value: &i64) -> NaivePosition {
        NaivePosition {
            x: self.x + value,
            y: self.y,
        }
    }
}

pub struct Position {
    pub x: i64,
    pub y: i64,
    pub aim: i64,
}

impl Position {
    fn up(&self, value: &i64) -> Position {
        Position {
            x: self.x,
            y: self.y,
            aim: self.aim - value,
        }
    }

    fn down(&self, value: &i64) -> Position {
        Position {
            x: self.x,
            y: self.y,
            aim: self.aim + value,
        }
    }

    fn forward(&self, value: &i64) -> Position {
        Position {
            x: self.x + value,
            y: self.y + value * self.aim,
            aim: self.aim,
        }
    }
}
