use anyhow::Result;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use itertools::Itertools;
use paris::info;
use crate::utils::get_input;

pub fn solve_1(filename: &str) -> Result<String> {
    let routes: Vec<Route> = get_input(filename);

    let answer = find_distances(routes)
        .iter()
        .min()
        .unwrap()
        .to_string();

    Ok(answer)
}

pub fn solve_2(filename: &str) -> Result<String> {
    let routes: Vec<Route> = get_input(filename);

    let answer = find_distances(routes)
        .iter()
        .max()
        .unwrap()
        .to_string();

    Ok(answer)
}


fn find_distances(routes: Vec<Route>) -> Vec<usize> {
    let cities: HashSet<_> = routes.iter().flat_map(|route| vec![route.from.clone(), route.to.clone()]).collect();

    info!("Cities: {cities:?}");
    let permutations = cities.iter().permutations(cities.len()).unique();

    let mut distances = vec![];

    for perm in permutations {

        let mut current_distance = 0;

        for res in perm.windows(2) {

            let first = &(*res.first().unwrap()).clone();
            let last = &(*res.last().unwrap()).clone();

            let path = routes
                .iter()
                .filter(|r| (r.from == *first || r.from == *last) && (r.to == *first || r.to == *last))
                .collect::<Vec<&Route>>();

            let a = path.first().unwrap();

            current_distance += a.distance;
        }

        distances.push(current_distance)
    }

    distances
}

#[derive(Hash, PartialEq, Eq)]
struct Route {
    from: String,
    to: String,
    distance: usize,
}

impl Display for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {} = {}", self.from, self.to, self.distance)
    }
}

impl FromStr for Route {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (cities, distance) = s.split_once(" = ").unwrap();
        let (from, to) = cities.split_once(" to ").unwrap();

        Ok(Route {
            from: from.to_string(),
            to: to.to_string(),
            distance: distance.parse().unwrap()
        })
    }
}