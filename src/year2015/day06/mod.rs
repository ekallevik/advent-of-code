use std::cmp::max;
use std::str::FromStr;
use std::string::ParseError;
use fancy_regex::Regex;
use crate::domain::position::Position2D;
use crate::utils::get_input;

const DIMENSIONS: usize = 1000;

pub fn solve_1(filename: &str) -> String {
    let instructions: Vec<SwitchInstruction> = get_input(filename)
        .into_iter()
        .map(|line: String| line.parse().unwrap())
        .collect();

    let grid = [[Switch(false); DIMENSIONS]; DIMENSIONS];

    do_commands::<Switch>(grid, instructions);

    let a: usize = grid
        .iter()
        .map(|row| count_switches(row))
        .sum();

    a.to_string()
}

fn count_switches(switches: &[Switch]) -> usize {
    switches.iter().filter(|row| row.0).count()
}


pub fn solve_2(filename: &str) -> String {
    let instructions: Vec<SwitchInstruction> = get_input(filename)
        .into_iter()
        .map(|line: String| line.parse().unwrap())
        .collect();

    let grid = [[Light(0); DIMENSIONS]; DIMENSIONS];

    do_commands::<Light>(grid, instructions);

    let a: isize = grid
        .iter()
        .map(|row| count_lights(row))
        .sum();

    a.to_string()
}

fn count_lights(lights: &[Light]) -> isize {
    lights.iter().map(|light| light.0).sum()
}

#[derive(Copy, Clone, Debug)]
struct Switch(bool);

impl Switchable for Switch {
    fn do_command(&mut self, command: &SwitchCommand) {
        match command {
            SwitchCommand::On => { self.0 = true }
            SwitchCommand::Off => { self.0 = false }
            SwitchCommand::Toggle => { self.0 = !self.0 }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Light(isize);

impl Switchable for Light {
    fn do_command(&mut self, command: &SwitchCommand) {
        match command {
            SwitchCommand::On => { self.0 += 1 }
            SwitchCommand::Off => { self.0 = max(0, self.0 - 1) }
            SwitchCommand::Toggle => { self.0 += 2 }
        }
    }
}

struct SwitchInstruction {
    first: Position2D,
    second: Position2D,
    command: SwitchCommand,
}

#[derive(Debug, Clone, Copy)]
enum SwitchCommand {
    On,
    Off,
    Toggle,
}

impl FromStr for SwitchCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(SwitchCommand::On),
            "turn off" => Ok(SwitchCommand::Off),
            "toggle" => Ok(SwitchCommand::Toggle),
            _ => Err(())
        }
    }
}

impl FromStr for SwitchInstruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([a-z\s]+)\s([\d,]+)\D+([\d,]+)").unwrap();
        let captures = re.captures(s).unwrap().unwrap();

        let command = captures.get(1).unwrap().as_str();
        let first = captures.get(2).unwrap().as_str();
        let second = captures.get(3).unwrap().as_str();

        Ok(SwitchInstruction {
            first: first.parse().unwrap(),
            second: second.parse().unwrap(),
            command: command.parse().unwrap(),
        })
    }
}

trait Switchable {
    fn do_command(&mut self, command: &SwitchCommand);
}

fn do_commands<T: Switchable>(mut grid: [[T; 1000]; 1000], instructions: Vec<SwitchInstruction>) {
    for SwitchInstruction { first, second, command: switch_command } in instructions {
        for x in first.0..(second.0 + 1) {
            for y in first.1..(second.1 + 1) {
                grid[x as usize][y as usize].do_command(&switch_command);
            }
        }
    };
}