use std::collections::VecDeque;
use std::str::FromStr;
use anyhow::Result;
use scan_fmt::scan_fmt;
use crate::utils::get_input_string;

#[derive(Debug)]
struct State {
    stacks: Vec<VecDeque<char>>,
}

impl State {

    fn pop(&mut self, index: usize) -> char {
        self.stacks[index - 1].pop_front().unwrap()
    }

    fn push(&mut self, index: usize, value: char) {
        self.stacks[index - 1].push_front(value)
    }

}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<_> = s.split('\n').collect();

        let cargo_width = (lines.first().unwrap().len() + 1) / 4 + 1;
        let mut stacks = vec![VecDeque::new(); cargo_width];

        for line in lines {
            for (index, char) in line.chars().enumerate() {
                if ('A'..='Z').contains(&char) {
                    let position = (index as f64 / 4.0).floor() as usize;
                    stacks[position].push_back(char);
                }
            }
        }

        Ok(State { stacks })
    }
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let target = "move {d} from {d} to {d}";
        if let Ok((amount, from, to)) = scan_fmt!(s, target, usize, usize, usize) {
            Ok(Instruction { amount, from, to })
        } else {
            Err(())
        }
    }
}

fn parse_input(filename: &str) -> Result<(State, Vec<Instruction>)> {
    let input = get_input_string(filename);
    let (state, instructions) = input.split_once("\n\n").unwrap();

    let state: State = state.parse().unwrap();

    let instructions: Vec<Instruction> = instructions
        .split('\n')
        .map(|i| i.parse().unwrap())
        .collect();

    Ok((state, instructions))
}

pub fn solve_1(filename: &str) -> Result<String> {

    let (mut state, instructions) = parse_input(filename)?;

    for Instruction { amount, from, to } in instructions {
        for _ in 0..amount {
            let cargo = state.pop(from);
            state.push(to, cargo);
        }
    }

    get_result(state)
}

pub fn solve_2(filename: &str) -> Result<String> {
    let (mut state, instructions) = parse_input(filename)?;

    for Instruction { amount, from, to } in instructions {
        let mut popped = VecDeque::new();
        for _ in 0..amount {
            let cargo = state.pop(from);
            popped.push_front(cargo);
        }

        for cargo in popped {
            state.push(to, cargo);
        }
    }

    get_result(state)
}

fn get_result(state: State) -> Result<String> {
    let res: Vec<_> = state.stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop_front())
        .map(|c| c as u8)
        .collect();

    Ok(String::from_utf8(res)?)
}