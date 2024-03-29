use anyhow::Result;
use std::collections::HashMap;

use crate::utils::get_input;
use itertools::Itertools;

fn parse_input(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut signals = vec![];
    let mut displays = vec![];

    for line in input.into_iter() {
        let split: Vec<&str> = line.split(" | ").collect();
        signals.push(split[0].to_string());
        displays.push(split[1].to_string());
    }

    (signals, displays)
}

fn parse_input2(input: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut signals = vec![];

    for line in input.into_iter() {
        let split: Vec<&str> = line.split(" | ").collect();

        let mut inputs: Vec<String> = split[0]
            .split_whitespace()
            .map(|elem| elem.chars().sorted().collect())
            .collect::<Vec<String>>();

        inputs.sort_by_key(|a| a.len());

        let outputs: Vec<String> = split[1]
            .split_whitespace()
            .map(|elem| elem.chars().sorted().collect())
            .collect();

        println!("Inputs: {:?} - outputs: {:?}", inputs, outputs);

        signals.push((inputs, outputs));
    }

    signals
}

pub fn solve_1(filename: &str) -> Result<String> {
    let input = get_input(filename);
    let (_, displays) = parse_input(input);

    Ok(displays
        .into_iter()
        .inspect(|s| println!("{:?}", s))
        .map(|segment| {
            segment
                .split_whitespace()
                .map(|s| match s.len() {
                    2 => 1,
                    3 => 1,
                    4 => 1,
                    7 => 1,
                    _ => 0,
                })
                .sum::<i64>()
        })
        .sum::<i64>()
        .to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input = get_input(filename);
    let signals = parse_input2(input);

    let mut sum = 0;

    for (signal, display) in signals {
        let mapping = decode(&signal);

        for (i, digit) in display.iter().enumerate() {
            let base: i32 = 10;
            let k = base.pow((3 - i) as u32);
            sum += k * (*mapping.get(&digit).unwrap() as i32);
        }
    }

    Ok(sum.to_string())
}

fn is_superset(signal: &str, encoding: &str) -> bool {
    encoding.chars().all(|c| signal.contains(c))
}

fn count_diff(signal: &str, encoding: &str) -> u8 {
    encoding.chars().filter(|&c| !signal.contains(c)).count() as u8
}

fn decode_base(signal: &String) -> Option<(&String, u8)> {
    match signal.len() {
        2 => Some((signal, 1)),
        3 => Some((signal, 7)),
        4 => Some((signal, 4)),
        7 => Some((signal, 8)),
        _ => None,
    }
}

fn decode(signals: &[String]) -> HashMap<&String, u8> {
    let mut decoding = HashMap::new();
    let mut encoding = HashMap::new();

    for signal in signals {
        match decode_base(signal) {
            None => {}
            Some((encoded, value)) => {
                decoding.insert(encoded, value);
                encoding.insert(value, encoded);
            }
        }
    }

    for signal in signals {
        if signal.len() == 6 {
            let contains_1 = is_superset(signal, *encoding.get(&1).unwrap());
            let contains_4 = is_superset(signal, *encoding.get(&4).unwrap());

            let (encoded, value) = match (contains_1, contains_4) {
                (_, true) => (signal, 9),
                (true, false) => (signal, 0),
                (false, false) => (signal, 6),
            };

            decoding.insert(encoded, value);
            encoding.insert(value, encoded);
        }
    }

    for signal in signals {
        if signal.len() == 5 {
            let diff_9 = count_diff(signal, encoding.get(&9).unwrap());

            let value = if diff_9 == 2 {
                2
            } else {
                let contains_1 = is_superset(signal, encoding.get(&1).unwrap());
                if contains_1 {
                    3
                } else {
                    5
                }
            };

            decoding.insert(signal, value);
            encoding.insert(value, signal);
        }
    }

    println!("Decoding: {:?}", decoding);
    println!("Encoding: {:?}", encoding);
    decoding
}
