use crate::utils::get_input_array;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Position = (usize, usize);
type Grid = Vec<Vec<usize>>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let neg_self = -(self.cost as i64);
        let neg_other = -(other.cost as i64);
        neg_self.cmp(&neg_other)
    }
}

pub fn solve_1(filename: &String) -> String {
    let input: Vec<Vec<usize>> = get_input_array(filename);

    let max_y = input.len();
    let max_x = input.first().unwrap().len();

    let result = a_star((0, 0), (max_x - 1, max_y - 1), &input);

    result.to_string()
}

pub fn solve_2(filename: &String) -> String {
    filename.to_string()
}

fn a_star(start: Position, goal: Position, grid: &Grid) -> usize {
    let mut open = BinaryHeap::new();
    open.push(State {
        cost: 0,
        position: (0, 0),
    });

    let mut current_scores = HashMap::new();
    current_scores.insert(start, 0);

    let mut estimated_total_scores = HashMap::new();
    estimated_total_scores.insert(start, calculate_h(&start, goal));

    while !open.is_empty() {
        let state = open.pop().unwrap();
        let current = state.position;
        println!("Searching from: {:?} (est. score={})", current, state.cost);

        if current.0 == goal.0 && current.1 == goal.1 {
            println!("FOUND GOAL (current={:?})", current);
            return *current_scores.get(&current).unwrap();
        }

        for neighbor in get_neighbors(&current, goal) {
            // todo
            let node_cost = grid[neighbor.1][neighbor.0];
            let candidate_g = current_scores.get(&current).unwrap() + node_cost;
            let present_g = *current_scores.entry(neighbor).or_insert(usize::MAX);

            if candidate_g < present_g {
                current_scores.insert(neighbor, candidate_g);
                estimated_total_scores.insert(neighbor, candidate_g + calculate_h(&neighbor, goal));
                open.push(State {
                    cost: candidate_g,
                    position: neighbor,
                });
            }
        }
    }

    unreachable!("A* failed...")
}

// todo: off by one x2?
// estimate for score further down
fn calculate_h(current: &Position, goal: Position) -> usize {
    goal.0 - current.0 + goal.1 - current.0 + 2
}

fn get_neighbors(current: &Position, max: (usize, usize)) -> Vec<Position> {
    let mut neighbors = vec![];

    if current.0 < max.0 {
        neighbors.push((current.0 + 1, current.1))
    };

    if current.1 < max.1 {
        neighbors.push((current.0, current.1 + 1))
    };

    neighbors
}

fn print_grid(grid: &[Vec<usize>]) {
    for (i, line) in grid.iter().enumerate() {
        println!("I={}: {:?}", i, line);
    }
}

fn print_nodes(input: &[(usize, usize)]) {
    for node in input {
        println!("{:?}", node);
    }
}
