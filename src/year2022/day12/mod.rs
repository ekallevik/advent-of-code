use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use anyhow::{Result};
use crate::utils::get_input;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position(usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Node {
    value: char,
    position: Position,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: Node,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let neg_self = self.cost as i64;
        let neg_other = other.cost as i64;
        neg_self.cmp(&neg_other).reverse()
    }
}

impl Node {
    // todo
    fn get_neighbors(&self, grid: &Vec<Vec<Node>>) -> Vec<Position> {
        let mut candidates = vec![];

        if self.position.0 > 0 {
            let new = Position(self.position.0 - 1, self.position.1);
            if is_walkable_neighbor(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        if self.position.1 > 0 {
            let new = Position(self.position.0, self.position.1 - 1);
            if is_walkable_neighbor(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        if self.position.0 < grid.len() - 1 {
            let new = Position(self.position.0 + 1, self.position.1);
            if is_walkable_neighbor(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        if self.position.1 < grid[0].len() - 1 {
            let new = Position(self.position.0, self.position.1 + 1);
            if is_walkable_neighbor(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        candidates
    }

    fn get_neighbors_2(&self, grid: &Vec<Vec<Node>>) -> Vec<Position> {
        let mut candidates = vec![];

        if self.position.0 > 0 {
            let new = Position(self.position.0 - 1, self.position.1);
            if is_walkable_neighbor_2(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        if self.position.1 > 0 {
            let new = Position(self.position.0, self.position.1 - 1);
            if is_walkable_neighbor_2(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        if self.position.0 < grid.len() - 1 {
            let new = Position(self.position.0 + 1, self.position.1);
            if is_walkable_neighbor_2(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        if self.position.1 < grid[0].len() - 1 {
            let new = Position(self.position.0, self.position.1 + 1);
            if is_walkable_neighbor_2(&self.position, &new, &grid) {
                candidates.push(new)
            }
        }

        candidates
    }
}

fn is_walkable_neighbor(from: &Position, to: &Position, grid: &Vec<Vec<Node>>) -> bool {
    is_walkable(grid[from.0][from.1].value, grid[to.0][to.1].value)
}

fn is_walkable_neighbor_2(from: &Position, to: &Position, grid: &Vec<Vec<Node>>) -> bool {
    is_walkable_2(grid[from.0][from.1].value, grid[to.0][to.1].value)
}

fn is_walkable(from: char, to: char) -> bool {
    (from == 'S' && to == 'a') || (to as i32 - from as i32 <= 1) || to == 'E'
}

fn is_walkable_2(from: char, to: char) -> bool {
    (from == 'S' && to == 'a') || (from as i32 - to as i32 <= 1) || to == 'E'
}

pub fn solve_1(filename: &str) -> Result<String> {
    let input: Vec<String> = get_input(filename);

    let mut start = None;
    let mut goal = None;

    let mut grid = vec![];

    for (i, line) in input.iter().enumerate() {
        let mut row = vec![];
        for (j, char) in line.chars().enumerate() {
            let node = if char == 'S' {
                let node1 = Node { value: 'a', position: Position(i, j) };
                start = Some(
                    node1
                );
                node1
            } else if char == 'E' {
                let node2 = Node { value: 'z', position: Position(i, j) };
                goal = Some(
                    node2
                );
                node2
            } else {
                Node { value: char, position: Position(i, j) }
            };

            row.push(node);
        }
        grid.push(row);
    }

    let ans = find_shortest_path(&grid, start.unwrap(), goal.unwrap());
    Ok(ans.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input: Vec<String> = get_input(filename);

    let mut start = None;

    let mut grid = vec![];

    for (i, line) in input.iter().enumerate() {
        let mut row = vec![];
        for (j, char) in line.chars().enumerate() {
            let node = if char == 'S' {
                Node { value: 'a', position: Position(i, j) }
            } else if char == 'E' {
                let node2 = Node { value: 'z', position: Position(i, j) };
                start = Some(
                    node2
                );
                node2
            } else {
                Node { value: char, position: Position(i, j) }
            };

            row.push(node);
        }
        grid.push(row);
    }

    println!("{start:?}");
    let ans = find_shortest_path_2(&grid, start.unwrap());
    Ok(ans.to_string())
}

fn find_shortest_path(grid: &Vec<Vec<Node>>, start: Node, goal: Node) -> usize {
    let mut open = BinaryHeap::new();
    open.push(State {
        cost: 0,
        node: start,
    });

    let mut current_scores = HashMap::new();
    current_scores.insert(start, 0);

    let mut estimated_total_scores = HashMap::new();
    estimated_total_scores.insert(start, calculate_h(&start.position, &goal.position));

    while !open.is_empty() {
        let state = open.pop().unwrap();
        let current = state.node;

        if current == goal {
            return *current_scores.get(&current).unwrap();
        }

        for neighbor in &current.get_neighbors(&grid) {
            let node_cost = 1;
            let neighbor_node = grid[neighbor.0][neighbor.1];
            let candidate_g = current_scores.get(&current).unwrap() + node_cost;
            let present_g = *current_scores.entry(neighbor_node).or_insert(usize::MAX);

            if candidate_g < present_g {
                current_scores.insert(neighbor_node, candidate_g);
                estimated_total_scores.insert(neighbor_node, candidate_g + calculate_h(&neighbor, &goal.position));
                open.push(State {
                    cost: candidate_g,
                    node: neighbor_node,
                });
            }
        }
    }

    unreachable!("A* failed...")
}

fn calculate_h(current: &Position, goal: &Position) -> usize {
    ((goal.0 as isize - current.0 as isize).abs() + (goal.1 as isize - current.1 as isize).abs()) as usize
}

fn find_shortest_path_2(grid: &Vec<Vec<Node>>, start: Node) -> usize {
    let mut open = BinaryHeap::new();
    open.push(State {
        cost: 0,
        node: start,
    });

    let mut current_scores = HashMap::new();
    current_scores.insert(start, 0);

    let mut estimated_total_scores = HashMap::new();
    estimated_total_scores.insert(start, 2000);

    while !open.is_empty() {
        let state = open.pop().unwrap();
        let current = state.node;

        if current.value == 'a' {
            return *current_scores.get(&current).unwrap();
        }

        for neighbor in &current.get_neighbors_2(&grid) {
            let node_cost = 1;
            let neighbor_node = grid[neighbor.0][neighbor.1];
            let candidate_g = current_scores.get(&current).unwrap() + node_cost;
            let present_g = *current_scores.entry(neighbor_node).or_insert(usize::MAX);

            if candidate_g < present_g {
                current_scores.insert(neighbor_node, candidate_g);
                estimated_total_scores.insert(neighbor_node, candidate_g + 1000);
                open.push(State {
                    cost: candidate_g,
                    node: neighbor_node,
                });
            }
        }
    }

    unreachable!("A* failed...")
}
