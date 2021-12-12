use crate::utils::get_input;
use paris::{info, warn};

type Connection = (String, String);
type Path = Vec<String>;
type Paths = Vec<Path>;

fn parse_input(filename: &String) -> Vec<Connection> {
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split('-').collect();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect::<Vec<(String, String)>>()
}
/*
pub fn solve_1_old(filename: &String) -> String {
    let input: Vec<Connection> = parse_input(filename);
    let starts: Vec<&Connection> = input.iter().filter(|&con| con.0 == "start").collect();

    let paths: Paths = starts
        .iter()
        .flat_map(|&start| find_paths_from_start(start, &input))
        .collect();

    warn!("{:?}", paths);
    paths.len().to_string()
}

 */

pub fn solve_1(filename: &String) -> String {
    let input: Vec<Connection> = parse_input(filename);

    let path: Path = vec!["start".to_string()];

    let paths: Paths = search(path, &input);

    warn!("{:?}", paths);
    paths.len().to_string()
}

fn search(path: Path, input: &Vec<Connection>) -> Vec<Path> {

    let current = path.last().unwrap();

    println!("Searching from: {}", current);

    if *current == "end" {
        return vec![path];
    }

    let neighbors: Vec<String> = input
        .iter()
        .filter(|&next| can_use_connection(current.clone(), next.clone(), &path))
        .map(|next| get_next_node(current.clone(), next.clone()))
        .inspect(|next| println!("next: {:?}", next))
        .collect();

    if neighbors.len() == 0 {
        return vec![];
    }

    input
        .iter()
        .filter(|&next| can_use_connection(current.clone(), next.clone(), &path))
        .map(|next| get_next_node(current.clone(), next.clone()))
        .inspect(|next| println!("next: {:?}", next))
        .flat_map(|next| {
            let mut new_path = path.clone();
            new_path.push(next);
            search(new_path, input)
        })
        .collect()
}

fn get_next_node(node: String, next: Connection) -> String {
    if node == next.0 {next.1} else {next.0}
}

fn can_use_connection(current_node: String, next: Connection, path: &Path) -> bool {
    if current_node == next.0 {
        is_available_cave(next.1, path)
    } else if current_node == next.1 {
        is_available_cave(next.0, path)
    } else {
        false
    }
}

fn is_small_cave(name: String) -> bool {
    name.chars().all(|c| c.is_lowercase())
}

fn is_available_cave(name: String, path: &Vec<String>) -> bool {
    let visit_count = path.iter().filter(|&con| con.clone() == name).count();

    match is_small_cave(name) {
        false => true,
        true => visit_count < 1,
    }
}

/*
fn find_paths_from_start(start: &Connection, input: &Vec<Connection>) -> Vec<Vec<String>> {
    let clone = start.clone();
    let current_path = vec![clone.0, clone.1];

    let node = start.clone().1;

    info!("Current path: {:?}", current_path);
    println!("Searching from: {:?}", node);

    let neighbors: Vec<&Connection> = input
        .iter()
        .filter(|con| is_possible(node.clone(), (*con).clone(), &current_path))
        .collect();

    println!("Neighbors: {:?}", neighbors);

    neighbors
        .iter()
        .flat_map(|&neighbor| {
            let clone = neighbor.clone();
            let n = if start == clone.0 {clone.1} else {clone.0} ;
            let mut temp_path = current_path.clone();
            temp_path.push(n);
            find_paths(temp_path, input)
        })
        .collect()
}



fn is_possible(from: String, to: Connection, path: &Path) -> bool {
    if from == to.0 {
        is_available_cave(to.1, path)
    } else if from == to.1 {
        is_available_cave(to.0, path)
    } else {
        false
    }
}

fn find_paths(
    current_path: Vec<String>,
    input: &Vec<Connection>,
) -> Vec<Vec<String>> {

    let node = current_path.last().unwrap();

    if node == "end" {
        let mut updated_path = current_path.clone();
        updated_path.push(node.clone());
        return vec![updated_path];
    }

    let neighbors: Vec<String> = input
        .iter()
        .filter(|&con|
                    {
                        let clone = con.clone();
                        let n = if node == clone.0 {clone.1} else {clone.0} ;
                        let mut temp_path = current_path.clone();
                        temp_path.push(n);
                        is_possible(node.clone(), con.clone(), &temp_path)

                    }
        )
        .map(|connection| {
            let conn = connection.clone();
            if node == connection.0 {
                conn.1
            } else {
                conn.0
            }
        })
        .collect();

    info!("Current path: {:?}", current_path);
    println!("Searching from {:?}", node);
    println!("Neighbors: {:?}", neighbors);

    let res = neighbors
        .iter()
        .flat_map(|neighbor| {
            let mut new_path = current_path.clone();
            new_path.push(node.clone());
            find_paths(neighbor.clone(), new_path, input)
        }
        )
        .collect();

    warn!("{:?}", res);
    res
}


 */
pub fn solve_2(filename: &String) -> String {
    filename.to_string()
}
