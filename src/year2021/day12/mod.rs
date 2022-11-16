use anyhow::Result;
use itertools::Itertools;

type Connection = (String, String);
type Path = Vec<String>;
type Paths = Vec<Path>;

fn parse_input(filename: &str) -> Vec<Connection> {
    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split('-').collect();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect::<Vec<(String, String)>>()
}

pub fn solve_1(filename: &str) -> Result<String> {
    let input: Vec<Connection> = parse_input(filename);
    let path: Path = vec!["start".to_string()];

    Ok(search(path, &input, &None).len().to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input: Vec<Connection> = parse_input(filename);
    let path: Path = vec!["start".to_string()];

    let small_caves: Vec<&String> = input
        .iter()
        .flat_map(|conn| vec![&(*conn).0, &(*conn).1])
        .filter(|&cave| is_small_cave(cave))
        .filter(|&cave| *cave != "start" && *cave != "end")
        .unique()
        .collect();

    let paths: Paths = small_caves
        .iter()
        .flat_map(|&cave| search(path.clone(), &input, &Some(cave)))
        .collect();

    // need to filter because special cave might be used 0, 1 or 2 times.
    Ok(paths.iter().unique().count().to_string())
}

fn search(path: Path, input: &[Connection], special_cave: &Option<&str>) -> Vec<Path> {
    let current = path.last().unwrap();

    if *current == "end" {
        return vec![path];
    }

    input
        .iter()
        .filter(|&next| can_use_connection(current, next, &path, special_cave))
        .map(|next| get_next_node(current, next))
        .map(|node| extend_path(&path, node))
        .flat_map(|path| search(path, input, special_cave))
        .collect()
}

fn extend_path(path: &[String], node: String) -> Path {
    let mut new_path = path.to_owned();
    new_path.push(node);
    new_path
}

fn can_use_connection(
    node: &str,
    next: &Connection,
    path: &[String],
    special_cave: &Option<&str>,
) -> bool {
    if *node == (*next).0 {
        is_available_cave(&(*next).1, path, special_cave)
    } else if *node == next.1 {
        is_available_cave(&(*next).0, path, special_cave)
    } else {
        false
    }
}

fn get_next_node(node: &str, next: &Connection) -> String {
    let clone = next.clone();
    if *node == (*next).0 {
        clone.1
    } else {
        clone.0
    }
}

fn is_available_cave(name: &str, path: &[String], special_cave: &Option<&str>) -> bool {
    let visit_count = path.iter().filter(|&node| *node == *name).count();

    match is_small_cave(name) {
        false => true,
        true => {
            if let Some(special_name) = *special_cave {
                if *special_name == *name {
                    visit_count < 2
                } else {
                    visit_count < 1
                }
            } else {
                visit_count < 1
            }
        }
    }
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(|c| c.is_lowercase())
}
