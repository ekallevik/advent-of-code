use std::str::FromStr;
use anyhow::Result;
use crate::utils::get_input;

enum Operation {
    Add(isize),
    Noop
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {

        let operation = if s == "noop" {
            Operation::Noop
        } else {
            let (_, value) = s.split_once(' ').unwrap();
            Operation::Add(value.parse().unwrap())
        };

        Ok(operation)
    }
}


pub fn solve_1(filename: &str) -> Result<String> {
    let operations: Vec<Operation> = get_input(filename);

    let mut cycle = 0;
    let mut registery = 1;
    let mut signal_strength: Vec<isize> = vec![];

    for op in operations {
        cycle += 1;
        update_signal_strength(&mut signal_strength, cycle, registery);

        match op {
            Operation::Add(value) => {
                cycle += 1;
                update_signal_strength(&mut signal_strength, cycle, registery);
                registery += value;
            },
            Operation::Noop => {}
        }
    }

    let sum: isize = signal_strength.iter().sum();
    Ok(sum.to_string())
}

fn update_signal_strength(signal_strength: &mut Vec<isize>, cycle: isize, registry: isize) {
    if cycle == 20 || ((cycle - 20) % 40 == 0 && cycle != 0) {
        signal_strength.push(cycle * registry)
    }
}

pub fn solve_2(filename: &str) -> Result<String> {
    let operations: Vec<Operation> = get_input(filename);

    let mut cycle: isize = 0;
    let mut registry = 1;

    let mut crt = "\n".to_string();

    for op in operations {

        cycle += 1;
        update_string(&mut crt, cycle, registry);

        match op {
            Operation::Add(value) => {
                cycle += 1;
                update_string(&mut crt, cycle, registry);
                registry += value;
            },
            Operation::Noop => {}
        }
    }

    Ok(crt)
}

fn update_string(res: &mut String, cycle: isize, registry: isize) {
    let pixel = cycle % 40;

    if (registry - pixel + 1).abs() <= 1 {
        res.push('#');
    } else {
        res.push('.');
    }

    if pixel == 0 && cycle != 0 {
        res.push('\n');
    }
}