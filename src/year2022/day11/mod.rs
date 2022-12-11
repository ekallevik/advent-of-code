use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use crate::domain::order::pop_max;
use crate::utils::get_input_string;


enum MathOperation {
    Plus(isize),
    Multiplication(isize),
    Power(u32),
}

impl MathOperation {
    fn apply(&self, item: isize) -> isize {
        match self {
            MathOperation::Plus(number) => item + number,
            MathOperation::Multiplication(number) => item * number,
            MathOperation::Power(number) => item.pow(*number),
        }
    }
}

impl Display for MathOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MathOperation::Plus(number) => write!(f, "+ {}", number),
            MathOperation::Multiplication(number) => write!(f, "* {}", number),
            MathOperation::Power(number) => write!(f, "^ {}", number),
        }
    }
}

impl FromStr for MathOperation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (first, second) = s.split_once("old ").unwrap().1.split_once(" ").unwrap();

        let op = if let Ok(number) = second.parse() {
            match first {
                "+" => MathOperation::Plus(number),
                "*" => MathOperation::Multiplication(number),
                _ => return Err(format!("Operation {first} is not supported"))
            }
        } else {
            MathOperation::Power(2)
        };

        Ok(op)
    }
}

struct Monkey {
    index: usize,
    starting_items: VecDeque<isize>,
    operation: MathOperation,
    divisor: isize,
    happy_path_monkey: usize,
    unhappy_path_monkey: usize,
}

impl Monkey {
    fn get_moves(&self, items: &mut VecDeque<isize>) -> Vec<(usize, isize)> {
        let mut moves = vec![];

        while let Some(item) = items.pop_front() {
            let applied = self.operation.apply(item) / 3;

            let recipient = if applied % self.divisor == 0 {
                self.happy_path_monkey
            } else {
                self.unhappy_path_monkey
            };

            moves.push((recipient, applied));
        }

        moves
    }

    fn get_moves2(&self, items: &mut VecDeque<isize>) -> Vec<(usize, isize)> {
        let mut moves = vec![];

        while let Some(item) = items.pop_front() {
            let applied = self.operation.apply(item);

            let recipient = if (applied % self.divisor) == 0 {
                self.happy_path_monkey
            } else {
                self.unhappy_path_monkey
            };

            moves.push((recipient, applied));
        }

        moves
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Monkey").unwrap();
        writeln!(f, "- Items: {:?}", self.starting_items).unwrap();
        writeln!(f, "- Operations: {}", self.operation).unwrap();
        writeln!(f, "- Divisor: {}", self.divisor).unwrap();
        writeln!(f, "\t- happy: {}", self.happy_path_monkey).unwrap();
        writeln!(f, "\t- unhappy: {}", self.unhappy_path_monkey).unwrap();
        Ok(())
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.split("\n").collect_vec();

        let index = scan_fmt!(lines[0], "Monkey {d}:", usize).unwrap();
        let starting_items = lines[1].split_once(": ").unwrap().1.split(", ").map(|c| c.parse().unwrap()).collect();
        let operation = lines[2].parse().unwrap();
        let divisor = scan_fmt!(lines[3], "Test: divisible by {d}", isize).unwrap();
        let happy_path_monkey = scan_fmt!(lines[4], "  If true: throw to monkey {d}", usize).unwrap();
        let unhappy_path_monkey = scan_fmt!(lines[5], "If false: throw to monkey {d}", usize).unwrap();

        Ok(Monkey {
            index,
            starting_items,
            operation,
            divisor,
            happy_path_monkey,
            unhappy_path_monkey,
        })
    }
}

fn parse_monkeys(filename: &str) -> Vec<Monkey> {
    get_input_string(filename)
        .split("\n\n")
        .map(|m| m.parse().unwrap())
        .collect()
}

pub fn solve_1(filename: &str) -> Result<String> {
    let monkeys = parse_monkeys(filename);

    let mut inspections = monkeys.iter().map(|_| 0).collect_vec();
    let mut catalog: HashMap<usize, VecDeque<_>> = monkeys
        .iter()
        .map(|m| (m.index, m.starting_items.clone()))
        .collect();

    for _ in 1..=20 {
        for monkey in &monkeys {
            let items = catalog.get_mut(&monkey.index).unwrap();
            let moves = monkey.get_moves(items);
            inspections[monkey.index] += moves.len();
            for (recipient_index, item) in moves {
                let recipient = catalog.get_mut(&recipient_index).unwrap();
                recipient.push_back(item);
            }
        }
    }

    let answer = pop_max(&mut inspections) * pop_max(&mut inspections);
    Ok(answer.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let monkeys = parse_monkeys(filename);

    let mut inspections = monkeys.iter().map(|_| 0).collect_vec();
    let mut catalog: HashMap<usize, VecDeque<_>> = monkeys
        .iter()
        .map(|m| (m.index, m.starting_items.clone()))
        .collect();

    let gcd = monkeys
        .iter()
        .map(|m| m.divisor)
        .fold(1, |acc, curr| acc * curr);

    for _ in 1..=10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let items = catalog.get_mut(&i).unwrap();
            let moves = monkey.get_moves2(items);
            inspections[i] += moves.len();

            for (recipient, item) in moves.into_iter() {
                let adjusted = adjust_item(item, &gcd);

                let recipient_items = catalog.get_mut(&recipient).unwrap();
                recipient_items.push_back(adjusted);
            }
        }
    }

    let answer = pop_max(&mut inspections) * pop_max(&mut inspections);
    Ok(answer.to_string())
}


fn adjust_item(item: isize, gcd: &isize) -> isize {
    let remainder = item % gcd;
    if remainder == 0 {
        item
    } else {
        remainder
    }
}
