use std::collections::HashSet;

use crate::utils::{get_input_array, PuzzlePart};

pub fn solve(part: PuzzlePart) -> u64 {
    println!("Puzzle day 09 - {:?}", part);
    let input = get_input_array::<u64>("src/input09.txt");

    match part {
        PuzzlePart::Part1 => solve_1(input),
        PuzzlePart::Part2 => solve_2(input),
    }
}

fn solve_1(depths: Vec<Vec<u64>>) -> u64 {
    find_low_points(&depths).iter().map(|&point| point.2 + 1).sum()
}

fn find_low_points(depths: &Vec<Vec<u64>>) -> Vec<(i32, i32, u64)> {

    let mut points = vec![];
    let row_size = depths.len();
    let col_size = depths.first().unwrap().len();

    for (r, row) in depths.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {

            let mut is_minimum = true;

            if r != 0 && depths[r-1][c] <= cell {
                is_minimum = false;
            }

            if r != row_size - 1 && depths[r+1][c] <= cell {
                is_minimum = false;
            }

            if c != 0 && depths[r][c-1] <= cell {
                is_minimum = false;
            }

            if c != col_size - 1 && depths[r][c+1] <= cell {
                is_minimum = false;
            }

            if is_minimum {
                println!("Found minimum value: {} at point: ({}, {})", depths[r][c], r, c);
                points.push((r as i32, c as i32, depths[r][c]))
            }
        }
    };

    points

}

fn generate_neighbors(x: i32, y: i32, x_size: i32, y_size: i32) -> Vec<(i32, i32)>{

    let mut res = vec![];

    if x != 0 {
        res.push((x-1, y))
    };

    if x != x_size - 1 {
        res.push((x+1, y))
    };

    if y != 0 {
        res.push((x, y-1))
    };

    if y != y_size - 1 {
        res.push((x, y+1))
    };

    res
}

fn solve_2(depths: Vec<Vec<u64>>) -> u64 {

    let origins = find_low_points(&depths);

    let x_size = (&depths).len() as i32;
    let y_size = (&depths).first().unwrap().len() as i32;

    let mut basins = vec![];


    for origin in origins {

        let mut candidates = vec![origin];
        let mut basin: HashSet<(i32, i32)> = vec![(origin.0, origin.1)].into_iter().collect();

        while !candidates.is_empty() {
            let candidate = candidates.pop().unwrap();
            println!("\nStarting search from value: {:?} ({:?}, {:?})", candidate.2, candidate.0, candidate.1);
            let neighbors = generate_neighbors(candidate.0, candidate.1, x_size, y_size);

            for neighbor in neighbors {

                let value: &u64 = &depths[neighbor.0 as usize][neighbor.1 as usize];
                if candidate.2 < *value && *value != 9 {
                    println!("Adding Neighbor with value {:?} ({:?}, {:?})", value, neighbor.0, neighbor.1);
                    let v = (neighbor.0, neighbor.1, *value);
                    candidates.push(v);
                    basin.insert((neighbor.0, neighbor.1));
                }

            }
            println!("Completed search from ({:?}, {:?})", candidate.0, candidate.1);
        }

        basins.push(basin);
    }

    let mut sorted = basins.iter().map(|basin| basin.len()).collect::<Vec<usize>>();
    sorted.sort_unstable();
    sorted.reverse();

    (sorted[0] * sorted[1] * sorted[2]) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let input = get_input_array::<u64>("src/input09_test.txt");

        let result = solve_1(input);
        assert_eq!(result, 15)
    }

    #[test]
    fn test_solve_2() {
        let input = get_input_array::<u64>("src/input09_test.txt");

        let result = solve_2(input);
        assert_eq!(result, 1134)
    }
}
