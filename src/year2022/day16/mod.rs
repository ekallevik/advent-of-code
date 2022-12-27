use std::collections::HashMap;
use std::str::FromStr;
use std::thread::current;
use anyhow::{bail, Result};
use itertools::Itertools;
use scan_fmt::scan_fmt;

use crate::utils::get_input;

#[derive(Debug, Hash)]
struct Valve {
    name: String,
    flow: usize,
    connections: Vec<String>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (definition, connection_string) = s.split_once("; ").unwrap();

        let definition_format = "Valve {} has flow rate={};";
        let (name, flow) = scan_fmt!(definition, definition_format, String, usize).unwrap();

        let connections = if connection_string.starts_with("tunnels") {
            let (_, connections) = connection_string.split_once("valves ").unwrap();
            connections.split(", ").map(|s| s.to_string()).collect()
        } else {
            let (_, connections) = connection_string.split_once("valve ").unwrap();
            vec![connections.to_string()]
        };


        println!("{:?}", connections);

        let valve = Valve { name, flow, connections };

        Ok(valve)
    }
}


pub fn solve_1(filename: &str) -> Result<String> {
    let valves: Vec<Valve> = get_input::<Valve>(filename);

    let valves: HashMap<String, &Valve> = valves.iter()
        .map(|v| (v.name.clone(), v))
        .collect();

    let minutes = 30;
    let start = valves["AA"];

    let mut max_flow = 0;

    for neighbor in &start.connections {
        println!("\n\nSearching from {} with best score={}\n", neighbor, max_flow);
        let current = valves[neighbor];

        let flow = search_valves(&valves, &vec![], start, current, minutes-1, max_flow, 0, 0);

        if flow > max_flow {
            max_flow = flow;
        }
    }

    Ok(max_flow.to_string())
}

fn search_valves(
    valves: &HashMap<String, &Valve>,
    open: &Vec<String>,
    previous: &Valve,
    current: &Valve,
    minutes: usize,
    current_best: usize,
    current_score: usize,
    current_flow: usize,
) -> usize {

    let mut best_score = current_best;

    if minutes == 0 {
        return current_score;
    }

    let estimated_best = estimate_score(valves, open, minutes, current_score, current_flow);

    if estimated_best < current_best {
        println!("Pruned at {minutes} minutes");
        return current_score;
    }

    let is_opening_current = current.name == previous.name;

    let mut updated = open.clone();
    let next_open = if is_opening_current {
        updated.push(current.name.clone());
        updated
    } else {
        updated
    };


    let new_flow = if is_opening_current {
        current_flow + current.flow
    } else {
        current_flow
    };

    let candidates = get_next_candidates(&current, previous, &next_open);
    let next_score = current_score + new_flow;

    let next_previous = current;
    for candidate in candidates {

        let next_current = valves[&candidate];

        let candidate_flows = search_valves(valves, &next_open, next_previous, next_current, minutes - 1, best_score, next_score, new_flow);

        if candidate_flows > best_score {
            best_score = candidate_flows;
        }
    }

    best_score
}

fn estimate_score(valves: &HashMap<String, &Valve>, open: &Vec<String>, minutes: usize, current_score: usize, current_flow: usize) -> usize {
    let mut closed: Vec<usize> = valves
        .values()
        .filter(|v| !open.contains(&v.name))
        .map(|v| v.flow)
        .sorted()
        .collect();

    let mut estimated_best: usize = current_score;
    let mut estimated_flow = current_flow;

    for i in 0..minutes {
        if i % 2 == 0 {
            if let Some(c) = closed.pop() {
                estimated_flow += c;
            }
        }
        estimated_best += estimated_flow;
    };
    estimated_best
}

fn get_next_candidates(current: &&Valve, previous: &Valve, open: &Vec<String>) -> Vec<String> {
    let neighbors = &current.connections;

    let mut candidates: Vec<String> = if current.name != previous.name {
        neighbors
            .iter()
            .filter(|&n| *n != previous.name)
            .map(|s| s.to_string())
            .collect()
    } else {
        neighbors.iter().map(|s| s.to_string()).collect()
    };

    if !open.contains(&current.name) && current.flow != 0 {
        candidates.push(current.name.clone());
    }

    candidates
}


pub fn solve_2(filename: &str) -> Result<String> {
    let valves: Vec<Valve> = get_input::<Valve>(filename);

    let valves: HashMap<String, &Valve> = valves.iter()
        .map(|v| (v.name.clone(), v))
        .collect();

    let minutes = 26;
    let start = valves["AA"];

    let mut max_flow = 0;

    // todo: remove hardcoding
    let neighbor_pairs = if filename.contains("real") {
        vec![
            (&start.connections[0], &start.connections[1]),
            (&start.connections[0], &start.connections[2]),
            (&start.connections[0], &start.connections[3]),
            (&start.connections[0], &start.connections[4]),
            (&start.connections[1], &start.connections[2]),
            (&start.connections[1], &start.connections[3]),
            (&start.connections[1], &start.connections[4]),
            (&start.connections[2], &start.connections[3]),
            (&start.connections[2], &start.connections[4]),
            (&start.connections[3], &start.connections[4]),
        ]
    } else {
        vec![
            (&start.connections[0], &start.connections[1]),
            (&start.connections[0], &start.connections[2]),
            (&start.connections[1], &start.connections[2]),
        ]
    };

    for (elf, elephant) in neighbor_pairs {
        println!("\n\nSearching from ({}, {}) with best score={}\n", elf, elephant, max_flow);
        let current_elf = valves[elf];
        let current_elephant = valves[elephant];

        let flow = search_valves_2(&valves, &vec![], (start, start), (current_elf, current_elephant), minutes-1, max_flow, 0, 0);

        if flow > max_flow {
            max_flow = flow;
        }
    }

    Ok(max_flow.to_string())
}

// todo: make generic
fn search_valves_2(
    valves: &HashMap<String, &Valve>,
    open: &Vec<String>,
    previous: (&Valve, &Valve),
    current: (&Valve, &Valve),
    minutes: usize,
    current_best: usize,
    current_score: usize,
    current_flow: usize,
) -> usize {

    let mut best_score = current_best;

    if minutes == 0 {
        println!(" 0 min - Ran out of time. Score: {current_score}, Best: {best_score}");
        return current_score;
    }

    let estimated_best = estimate_score_2(valves, open, minutes, current_score, current_flow);

    if estimated_best < current_best {
        return current_score;
    }

    let elf_is_opening = current.0.name == previous.0.name;
    let elephant_is_opening = current.1.name == previous.1.name;

    // todo: simplify...
    let mut new_flow = current_flow;
    let mut next_open = open.clone();
    if elf_is_opening {
        next_open.push(current.0.name.clone());
        new_flow += current.0.flow
    };

    if elephant_is_opening {
        next_open.push(current.1.name.clone());
        new_flow += current.1.flow
    };

    let next_score = current_score + new_flow;
    let elf_candidates = get_next_candidates(&current.0, previous.0, &next_open);
    let elephant_candidates = get_next_candidates(&current.1, previous.1, &next_open);

    let next_previous = current;

    for elf_candidate in &elf_candidates {
        for elephant_candidate in &elephant_candidates {


            if elf_candidate == elephant_candidate {
                continue
            }

            let next_current = (
                valves[elf_candidate],
                valves[elephant_candidate],
            );

            let candidate_flows = search_valves_2(valves, &next_open, next_previous, next_current, minutes - 1, best_score, next_score, new_flow);

            if candidate_flows > best_score {
                best_score = candidate_flows;
            }

        }


    }

    best_score
}

fn estimate_score_2(valves: &HashMap<String, &Valve>, open: &Vec<String>, minutes: usize, current_score: usize, current_flow: usize) -> usize {
    let mut closed: Vec<usize> = valves
        .values()
        .filter(|v| !open.contains(&v.name))
        .map(|v| v.flow)
        .sorted()
        .collect();

    let mut estimated_best: usize = current_score;
    let mut estimated_flow = current_flow;

    for i in 0..minutes {
        if i % 2 == 0 {
            if let Some(c) = closed.pop() {
                estimated_flow += c;
            }
            if let Some(c) = closed.pop() {
                estimated_flow += c;
            }
        }
        estimated_best += estimated_flow;
    };
    estimated_best
}
