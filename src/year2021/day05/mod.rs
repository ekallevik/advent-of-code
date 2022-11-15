use std::collections::HashMap;
use crate::domain::NaiveLine;
use crate::utils::get_input;

pub fn solve_1(filename: &str) -> String {
    let input: Vec<NaiveLine> = get_input(filename);

    let lines = input
        .into_iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect::<Vec<NaiveLine>>();

    map_ocean_vents(lines).to_string()
}


pub fn solve_2(filename: &str) -> String {
    let input: Vec<NaiveLine> = get_input(filename);

    map_ocean_vents(input).to_string()
}

fn map_ocean_vents(lines: Vec<NaiveLine>) -> usize {
    let mut ocean_map = HashMap::new();

    // todo: cleanup
    for line in lines {
        if line.start.x == line.end.x {
            let x = line.start.x as usize;

            let y_range = if line.start.y < line.end.y {
                line.start.y..=line.end.y
            } else {
                line.end.y..=line.start.y
            };

            for y in y_range {
                *ocean_map.entry((x as usize, y as usize)).or_insert(0) += 1;
            }
        } else if line.start.y == line.end.y {
            let y = line.start.y as usize;

            let x_range = if line.start.x < line.end.x {
                line.start.x..=line.end.x
            } else {
                line.end.x..=line.start.x
            };

            for x in x_range {
                *ocean_map.entry((x as usize, y as usize)).or_insert(0) += 1;
            }
        } else {
            let points = line.to_diagonal_vec();
            for (x, y) in points {
                *ocean_map.entry((x as usize, y as usize)).or_insert(0) += 1;
            }
        }
    }

    print_ocean_floor(&ocean_map);
    ocean_map.iter().filter(|(_, v)| **v >= 2).count()
}

//todo: generalize?
fn print_ocean_floor(ocean: &HashMap<(usize, usize), i64>) {

    for i in 0..10 {
        let mut row = String::new();
        for j in 0..10 {
            if let Some(value) = ocean.get(&(j as usize, i as usize)) {
                row = row + " " + &*value.to_string();
            } else {
                row += &*"  ".to_string();

            }
        }
        println!("{:?}", row);
    }
}

