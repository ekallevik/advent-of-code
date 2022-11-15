use std::collections::HashMap;
use std::fmt::{Display, Formatter};

struct Dimensions(usize, usize);

#[derive(Debug, Clone, Copy)]
enum SeaCucumber {
    East,
    South,
}

impl Display for SeaCucumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SeaCucumber::East => write!(f, ">"),
            SeaCucumber::South => write!(f, "v"),
        }
    }
}

fn parse_sea_cucumber(char: char) -> Option<SeaCucumber> {
    match char {
        'v' => Some(SeaCucumber::South),
        '>' => Some(SeaCucumber::East),
        '.' => None,
        _ => panic!("Should not happen"),
    }
}

fn parse_mariana_trench(input: &str) -> (HashMap<(usize, usize), SeaCucumber>, Dimensions) {
    let mut trench = HashMap::new();

    let mut dims = Dimensions(0, 0);
    let mut i_size = 0;

    for (i, line) in input.lines().enumerate() {
        i_size += 1;
        let j_size = line.len();
        dims.1 = j_size;

        for (j, elem) in line.chars().enumerate() {
            match parse_sea_cucumber(elem) {
                None => {}
                Some(cucumber) => { trench.insert((i, j), cucumber); }
            }
        }
    }

    dims.0 = i_size;

    (trench, dims)
}


pub fn solve_1(filename: &str) -> String {
    let input = std::fs::read_to_string(filename).expect("file not found!");
    let (mut current, dims) = parse_mariana_trench(&input);

    let east_target = &SeaCucumber::East;
    let south_target = &SeaCucumber::South;

    let mut iteration = 0;
    loop {
        println!("Iteration: {}", iteration);
        iteration += 1;

        let (moved_east, has_moved_east) = move_cucumbers(current, &dims, east_target);
        let (moved_south, has_moved_south) = move_cucumbers(moved_east, &dims, south_target);

        current = moved_south;

        if !has_moved_east && !has_moved_south {
            println!("\nFinal trench");
            print_trench(&current, &dims);
            break;
        }
    }

    iteration.to_string()
}

pub fn solve_2(_: &str) -> String {
    let number_of_stars = 37;
    number_of_stars.to_string()
}

fn move_cucumbers(current: HashMap<(usize, usize), SeaCucumber>, dims: &Dimensions, target: &SeaCucumber) -> (HashMap<(usize, usize), SeaCucumber>, bool) {
    let mut has_moved = false;
    let mut next = HashMap::new();

    for (current_pos, cucumber) in current.iter() {
        let insert_pos = match (target, cucumber)  {
            (SeaCucumber::South, SeaCucumber::East) | (SeaCucumber::East, SeaCucumber::South) => {
                *current_pos
            }
            (SeaCucumber::East, SeaCucumber::East) => {
                let next_coordinate = (current_pos.0 % dims.0, (current_pos.1 + 1) % dims.1);
                if !current.contains_key(&next_coordinate) {
                    has_moved = true;
                    next_coordinate
                } else {
                    *current_pos
                }
            }
            (SeaCucumber::South, SeaCucumber::South) => {
                let next_coordinate = ((current_pos.0 + 1) % dims.0, current_pos.1 % dims.1);
                if !current.contains_key(&next_coordinate) {
                    has_moved = true;
                    next_coordinate
                } else {
                    *current_pos
                }
            }
        };
        next.insert(insert_pos, *cucumber);
    }

    (next, has_moved)
}

fn print_trench(trench: &HashMap<(usize, usize), SeaCucumber>, dims: &Dimensions) {
    for i in 0..dims.0 {
        let mut row_s = String::with_capacity(dims.1);
        for j in 0..dims.1 {
            match trench.get(&(i, j)) {
                None => row_s += ".",
                Some(value) => row_s += &*value.to_string().clone()
            }
        }
        println!("{}", row_s);
    }
}
