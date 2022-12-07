use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use anyhow::{Result};
use crate::utils::{get_input};


#[derive(Debug)]
enum Elem {
    Dir(PathBuf, Vec<String>),
    File(PathBuf, usize),
}

impl Display for Elem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Dir(name, content) => writeln!(f, "{:?}-{:?}", name, content),
            Elem::File(name, size) => writeln!(f, "{:?}:{}", name, size)
        }
    }
}

fn parse_elem(current: &mut PathBuf, line: &String) -> Elem {
    let (value, name) = line.split_once(" ").unwrap();
    let new = current.join(name);

    let elem = if line.starts_with("dir") {
        Elem::Dir(new, vec![])
    } else {
        Elem::File(new, value.parse().unwrap())
    };
    elem
}

pub fn solve_1(filename: &str) -> Result<String> {
    let input: Vec<String> = get_input(filename);

    let catalog = catalog_files(input);
    let sizes = resolve_catalog_size(&catalog);

    let answer: usize = sizes
        .values()
        .filter(|&size| *size <= 100_000)
        .sum();

    Ok(answer.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input: Vec<String> = get_input(filename);

    let catalog = catalog_files(input);
    let sizes = resolve_catalog_size(&catalog);

    let used_size = sizes.get(&get_root()).unwrap();
    let diff_size = used_size - 40_000_000;

    let answer = sizes
        .values()
        .filter(|&s| *s >= diff_size)
        .min()
        .unwrap();

    Ok(answer.to_string())
}


fn catalog_files(input: Vec<String>) -> HashMap<PathBuf, Vec<Elem>> {
    let mut directories: HashMap<PathBuf, Vec<Elem>> = HashMap::new();
    directories.insert(get_root(), vec![]);

    let mut current = Path::new(".").to_owned();

    for line in input {
        if !line.starts_with("$") {
            let elem = parse_elem(&mut current, &line);

            if let Some(parent) = directories.get_mut(&current) {
                parent.push(elem);
            } else {
                match elem {
                    Elem::Dir(path, _) => directories.insert(path, vec![]),
                    Elem::File(_, _) => panic!("Cannot insert file into dir: {current:?}")
                };
            }
        } else if line.starts_with("$ cd") {
            let (_, dir) = line.rsplit_once(" ").unwrap();

            if dir == "/".to_string() {
                current = get_root();
            } else if dir == ".." {
                current = current.parent().unwrap().to_path_buf();
            } else {
                let buf = current.join(Path::new(dir));
                directories.insert(buf.clone(), vec![]);
                current = buf;
            }
        } else {
            assert_eq!(line, "$ ls");
        }
    }

    directories
}

fn resolve_catalog_size(catalog: &HashMap<PathBuf, Vec<Elem>>) -> HashMap<PathBuf, usize> {
    let mut sizes = HashMap::new();
    resolve_catalog_size_memoized(&catalog, get_root(), &mut sizes);
    sizes
}

fn resolve_catalog_size_memoized(
    directories: &HashMap<PathBuf, Vec<Elem>>,
    dir: PathBuf,
    sizes: &mut HashMap<PathBuf, usize>,
) -> usize {
    if let Some(value) = sizes.get(&dir) {
        *value
    } else {
        let elems = directories.get(&dir).unwrap();

        let sum = elems
            .iter()
            .map(|e| match e {
                Elem::Dir(name, _) =>
                    {
                        resolve_catalog_size_memoized(&directories, dir.join(name), sizes)
                    }
                Elem::File(_, size) => *size
            }
            )
            .sum();

        sizes.insert(dir, sum);
        sum
    }
}

fn get_root() -> PathBuf {
    Path::new("/").to_path_buf()
}