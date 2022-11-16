use anyhow::Result;
use crate::utils::get_input_array;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Position = (usize, usize);

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
        let neg_self = self.cost as i64;
        let neg_other = other.cost as i64;
        neg_self.cmp(&neg_other).reverse()
    }
}

pub fn solve_1(filename: &str) -> Result<String> {
    let input: Vec<Vec<usize>> = get_input_array(filename);

    let max_y = input.len();
    let max_x = input.first().unwrap().len();

    Ok(a_star((0, 0), (max_x - 1, max_y - 1), &input).to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input: Vec<Vec<usize>> = get_input_array(filename);

    let max_y = input.len() * 5;
    let max_x = input.first().unwrap().len() * 5;

    Ok(a_star((0, 0), (max_x - 1, max_y - 1), &input).to_string())
}

fn a_star(start: Position, goal: Position, grid: &[Vec<usize>]) -> usize {
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

        if current == goal {
            return *current_scores.get(&current).unwrap();
        }

        for neighbor in get_neighbors(&current, goal) {
            let node_cost = get_grid_value(grid, neighbor);
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

fn get_grid_value(grid: &[Vec<usize>], position: Position) -> usize {
    let max_y = grid.len();
    let max_x = grid.first().unwrap().len();

    let increment = get_increment(position, max_x, max_y);

    let node_cost = grid[position.0 % max_x][position.1 % max_y];
    if increment == 0 {
        node_cost
    } else {
        let value = node_cost + increment;
        if value <= 9 {
            value
        } else {
            value % 10 + 1
        }
    }
}

fn get_increment(position: Position, max_x: usize, max_y: usize) -> usize {
    let increment_x = position.0 / (max_x);
    let increment_y = position.1 / (max_y);

    increment_x + increment_y
}

fn calculate_h(current: &Position, goal: Position) -> usize {
    goal.0 - current.0 + goal.1 - current.1
}

fn get_neighbors(current: &Position, max: (usize, usize)) -> Vec<Position> {
    let mut neighbors = vec![];

    if current.0 < max.0 {
        neighbors.push((current.0 + 1, current.1))
    };

    if current.1 < max.1 {
        neighbors.push((current.0, current.1 + 1))
    };

    if current.0 > 0 {
        neighbors.push((current.0 - 1, current.1))
    };

    if current.1 > 0 {
        neighbors.push((current.0, current.1 - 1))
    };

    neighbors
}


#[cfg(test)]
mod tests {
    use crate::utils::get_input_array;
    use crate::year2021::day15::{get_grid_value, get_increment};

    #[test]
    fn test_get_grid_value() {
        let input: Vec<Vec<usize>> = get_input_array("src/day15/test.txt");

        assert_eq!(get_grid_value(&input, (0, 0)), 1);
        assert_eq!(get_grid_value(&input, (9, 9)), 1);
        assert_eq!(get_grid_value(&input, (10, 0)), 2);
        assert_eq!(get_grid_value(&input, (0, 10)), 2);
        assert_eq!(get_grid_value(&input, (10, 10)), 3);
    }

    #[test]
    fn test_get_tile() {
        let tile = get_increment((10, 10), 10, 10);

        assert_eq!(tile, 2);
    }

    #[test]
    fn test_wrap() {

        let initial = 8;
        let expected = [8, 9, 1, 2, 3, 4, 5, 6, 7];

        for (inc, exp) in expected.into_iter().enumerate() {
            println!("{}", inc);
            let actual = if initial + inc <= 9 {initial+inc} else {(initial + inc) % 10 +1};
            assert_eq!(actual, exp)
        }
    }
}
