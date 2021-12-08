
use std::collections::HashMap;

use crate::utils::{get_input, PuzzlePart};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solve(part: PuzzlePart) -> u64 {
    println!("Puzzle day 08 - {:?}", part);

    let input = get_input("src/input08.txt");
    let input2 = get_input("src/input08.txt");
    let (_, displays) = parse_input(input);
    let signals = parse_input2(input2);

    match part {
        PuzzlePart::Part1 => solve_part_1(displays) as u64,
        PuzzlePart::Part2 => solve_part_2(signals) as u64,
    }
}

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

fn solve_part_1(display: Vec<String>) -> i64 {
    

    display
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
}

fn solve_part_2(signals: Vec<(Vec<String>, Vec<String>)>) -> i64 {
    let mut sum = 0;

    for (signal, display) in signals {
        //println!("Display: {:?}", display);

        let mapping = decode(&signal);

        //println!("Mapping {:?}", mapping);

        for (i, digit) in display.iter().enumerate() {
            println!("{:?}", digit);
            let base: i32 = 10;
            let k = base.pow((3 - i) as u32);
            sum += k * (*mapping.get(&digit).unwrap() as i32);
        }
    }

    sum as i64
}

/*
fn insert(decoding: &mut HashMap<&str, u8>, encoding: &mut HashMap<u8, &str>, signal: &str, value: u8) {

    let mut chars: Vec<char> = signal.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let s = String::from_iter(chars);

    (&decoding).insert(signal, value);
    (&encoding).insert(value, signal);
}

 */

fn is_superset(signal: &String, encoding: &String) -> bool {
    encoding.chars().all(|c| signal.contains(c))
}

fn count_diff(signal: &String, encoding: &String) -> u8 {
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

fn decode(signals: &Vec<String>) -> HashMap<&String, u8> {
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

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_solve_part_1() {
        let input = get_input("src/input08_test2.txt");
        let (_, displays) = parse_input(input);

        let expected = 26;

        let result = solve_part_1(displays);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_2_1() {
        let input = get_input("src/input08_test1.txt");
        let signals = parse_input2(input);

        let expected = 5353;

        let result = solve_part_2(signals);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_solve_part_2_2() {
        let input = get_input("src/input08_test2.txt");
        let signals = parse_input2(input);

        let expected = 61229;

        let result = solve_part_2(signals);
        assert_eq!(result, expected)
    }
}
