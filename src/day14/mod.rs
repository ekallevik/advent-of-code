use itertools::Itertools;

use std::collections::HashMap;

use std::time::Instant;

type Pair = (char, char);
type Rules = HashMap<Pair, char>;
type PairFreq = HashMap<Pair, usize>;

// todo: clean up this
// into_once
// better parsing
// immutables
fn parse_input(filename: &str) -> (PairFreq, Rules) {
    let mut frequencies: PairFreq = HashMap::new();
    let mut rules = HashMap::new();

    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .for_each(|line| {
            if line.contains("->") {
                let split: Vec<&str> = line.split(" -> ").collect();

                let key: Pair = (
                    split[0].to_string().chars().collect::<Vec<char>>()[0],
                    split[0].to_string().chars().collect::<Vec<char>>()[1],
                );
                let value: char = split[1].to_string().chars().collect::<Vec<char>>()[0];

                rules.insert(key, value);
            } else if line.trim() != "" {
                let pairs: Vec<Pair> = line.chars().tuple_windows().collect::<Vec<Pair>>();

                for pair in pairs.iter() {
                    if frequencies.contains_key(pair) {
                        let current = frequencies.get(pair).unwrap();
                        frequencies.insert(*pair, current + 1);
                    } else {
                        frequencies.insert(*pair, 1);
                    }
                }
            }
        });

    (frequencies, rules)
}

pub fn solve_1(filename: &str) -> String {
    let (frequencies, rules) = parse_input(filename);
    solve_problem(&rules, frequencies, 10).to_string()
}

pub fn solve_2(filename: &str) -> String {
    let start = Instant::now();
    let (frequencies, rules) = parse_input(filename);
    let res = solve_problem(&rules, frequencies, 40).to_string();
    let duration = start.elapsed();
    println!("{:?}", duration);
    res
}

fn solve_problem(rules: &Rules, mut frequencies: PairFreq, steps: i8) -> usize {
    for _ in 0..steps {
        frequencies = iterate(rules, frequencies);
    }

    let mut result: HashMap<char, usize> = HashMap::new();
    for (pair, freq) in frequencies {
        *result.entry(pair.0).or_insert(0) += freq;
    }

    let max = result.iter().max_by_key(|&entry| entry.1).unwrap();
    let min = result.iter().min_by_key(|&entry| entry.1).unwrap();

    println!("{:?}", result);

    // todo: off by one??
    max.1 - min.1 + 1
}

// todo: cleanup
fn iterate(rules: &Rules, pair_freq: PairFreq) -> PairFreq {
    // todo: make this a generic func?
    let mut frequencies: PairFreq = HashMap::new();
    for (pair, freq) in pair_freq {
        let (left, right) = apply_rule(rules, pair);

        *frequencies.entry(left).or_insert(0) += freq;
        *frequencies.entry(right).or_insert(0) += freq;
    }

    frequencies
}

fn apply_rule(rules: &Rules, pair: Pair) -> (Pair, Pair) {
    let &insert = rules.get(&pair).unwrap();
    ((pair.0, insert), (insert, pair.1))
}
