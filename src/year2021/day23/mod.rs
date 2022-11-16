// todo: fix typo

/*
THE RULES OF THE GAME
1. each move cost energy
2. can only move into open space
3. can only move Manhattan-style
4. the Amphipods should be sorted A-D left to right
5. Amphipods will never stop just outside a room, but can pass through
6. Amphipods will only move into the room that it it's finally destination
    & it only contains Amphipods of the correct type
7. Once moved into the hallway, it will stay there til it can move directly to destination
*/
use anyhow::Result;
use core::fmt;
use std::{fmt::{Display, Formatter}, str::FromStr};

pub fn solve_1(_: &str) -> Result<String> {

    let locations = vec![
        Location::new_hallway(0),
        Location::new_hallway(1),
        Location::new_room('A', 'B', 'A', 2),
        Location::new_hallway(3),
        Location::new_room('D', 'C', 'B', 4),
        Location::new_hallway(5),
        Location::new_room('C', 'B', 'C', 6),
        Location::new_hallway(7),
        Location::new_room('A', 'D', 'D', 8),
        Location::new_hallway(9),
        Location::new_hallway(10),
    ];

    Ok(solve(locations, 0, 0).unwrap().to_string())
}

pub fn solve_2(_filename: &str) -> Result<String> {
    todo!()
}

fn solve(locations: Vec<Location>, iteration: usize, branch: usize) -> Option<usize> {

    let mut costs = vec![];
    let candidates: Vec<usize> = locations.iter().filter(|loc| loc.has_movable()).map(|loc| loc.position()).collect();

    println!("\n### START ITERATION {}-{} ###", iteration, branch);
    print_locations(&locations);
    //println!("Candidates: {:?}", candidates);

    //let mut is_correct = String::new();
    //std::io::stdin().read_line(&mut is_correct).unwrap();


    // todo: if only one step left only the target room is available

    for candidate in candidates {
        let position = candidate;
        let peek = locations.get(candidate).unwrap().peek().unwrap();

        // todo: maybe add unit tests

        let targets = find_targets(&locations, position, peek);

        // todo: could calculate score earlier
        if targets.is_empty() {
            println!("Calculating cost for iteration: {}", iteration);
            print_locations(&locations);
            return calculate_score(&locations);
        }

        for target in targets {
            let candidate_cost = apply_move(locations.clone(), candidate, target.position(), iteration, target.position());

            println!("Pushing cost: {:?}", candidate_cost);
            costs.push(candidate_cost);
        }
    }

    println!("### END ITERATION {} ###", iteration);

    costs.into_iter().flatten().min()
}

fn find_targets<'a>(locations: &'a [Location], position: usize, peek: &Amphipod) -> Vec<&'a Location> {

    let occupied_halls: Vec<&Location> = locations
        .iter()
        .filter(|loc| matches!(loc, Location::Hallway(_, _)))
        .filter(|loc| loc.available_spaces() == 0)
        .filter(|loc| loc.position() != position)
        .collect();

    // inclusive
    let left_block = occupied_halls
        .iter()
        .filter(|block| block.position() < position)
        .max_by_key(|block| block.position())
        .map_or(0, |block| block.position());

    let right_block = occupied_halls
        .iter()
        .filter(|block| block.position() > position)
        .min_by_key(|block| block.position())
        .map_or(11, |block| block.position()-1);

    let targets: Vec<&Location> = locations
        .iter()
        .filter(|loc| loc.available(peek))
        .filter(|loc| loc.position() != position)
        .filter(|loc| loc.position() >= left_block)
        .filter(|loc| loc.position() <= right_block)
        .collect();

    println!("Occupied - left={}, right={}, pos={}", left_block, right_block, position);
    print_locations2(&occupied_halls);
    //let mut is_correct = String::new();
    //std::io::stdin().read_line(&mut is_correct).unwrap();

    //println!("Targets");
    //print_locations2(&targets);
    targets
}

fn calculate_score(locations: &[Location]) -> Option<usize> {

    let mut score = 0;

    let rooms = locations
        .iter()
        .filter(|loc| matches!(loc, Location::Room(_, _, _)))
        .collect::<Vec<&Location>>();

    for room in rooms {
        if room.current_occupancy_is_correct() {
            score += room.get_score();
        } else {
            return None
        }
    }

    println!("\n### START ###\n");
    println!("Found score: {:?}", score);
    print_locations(locations);
    println!("\n### END ###\n");

    Some(score)
}

fn print_locations(locations: &[Location]) {
    for candidate in locations.iter() {
        println!("{}", candidate)
    }
}

fn print_locations2(locations: &[&Location]) {
    for candidate in locations.iter() {
        println!("{}", candidate)
    }
}

fn apply_move(mut locs: Vec<Location>, from: usize, to: usize, iteration: usize, branch: usize) -> Option<usize> {

    let from = locs.get_mut(from).unwrap();
    let from_pos = from.position() as isize;
    let (pod, up_steps) = from.pop();

    let to = locs.get_mut(to).unwrap();
    let sideways_steps = to.position() - from_pos.unsigned_abs();

    let pod = pod.unwrap();
    let steps: usize = up_steps + sideways_steps;

    to.push_and_update(pod, steps);

    solve(locs, iteration+1, branch)
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Location {
    // todo: change to Option<Amphipod>?
    Hallway(Vec<Amphipod>, usize),
    Room(Vec<Amphipod>, Amphipod, usize),
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Location::Hallway(pods, pos) => {
                if let Some(pod) = pods.first() {
                    write!(f, "Hall {}: {}", pos, pod)
                } else {
                    write!(f, "Hall {}: Ø", pos)
                }
            }
            Location::Room(pods, target, _) => {
                let target = match target {
                    Amphipod::Amber(_) => "A",
                    Amphipod::Bronze(_) => "B",
                    Amphipod::Copper(_) => "C",
                    Amphipod::Dessert(_) => "D"
                };

                match pods.len() {
                    0 => write!(f, "Room {}: Ø", target),
                    1 => write!(f, "Room {}: {}", target, pods.first().unwrap()),
                    2 => write!(f, "Room {}: {}, {}", target, pods.first().unwrap(), pods.last().unwrap()),
                    _ => write!(f, "THIS IS A BUG")
                }
            }
        }
    }
}

impl Location {
    pub fn new_hallway(position: usize) -> Location {
        Location::Hallway(vec![], position)
    }
    // todo: cleanup
    pub fn new_room(bottom: char, top: char, target: char, position: usize) -> Location {
        let bottom = bottom.to_string().parse().unwrap();
        let top = top.to_string().parse().unwrap();
        let target = target.to_string().parse().unwrap();

        Location::Room(vec![bottom, top], target, position)
    }

    pub fn available(&self, pod: &Amphipod) -> bool {
        match self {
            Location::Hallway(_, _) => self.available_spaces() > 0 && !pod.has_moved(),
            Location::Room(_, target, _) => {
                self.available_spaces() > 0 && (*pod == *target)
            }
        }
    }

    pub fn available_spaces(&self) -> usize {
        match self {
            Location::Hallway(pods, _) => 1 - pods.len(),
            Location::Room(pods, _, _) => 2 - pods.len(),
        }
    }

    pub fn position(&self) -> usize {
        match self {
            Location::Hallway(_, pos) => *pos,
            Location::Room(_, _, pos) => *pos,
        }
    }

    pub fn get_score(&self) -> usize {
        match self {
            Location::Hallway(_, _) => 0,
            Location::Room(pods, target, _) => {
                if *target != *pods.first().unwrap() {
                    panic!("Should not happen")
                }

                pods.iter().map(|pod| pod.cost()).sum()
            }
        }
    }

    pub fn push_and_update(&mut self, mut pod: Amphipod, accumulated_steps: usize) -> usize {
        if self.available_spaces() == 0 {
            panic!("Should not happen");
        }

        let spaces = self.available_spaces();
        match self {
            Location::Hallway(pods, _) => {
                pod.add_steps(accumulated_steps);
                pods.push(pod);
                0
            }
            Location::Room(pods, _, _) => {
                pod.add_steps(accumulated_steps + spaces);
                pods.push(pod);
                self.available_spaces() + 1
            }
        };

        0
    }

    pub fn pop(&mut self) -> (Option<Amphipod>, usize) {
        println!("Before pop: {}", self.clone());
        match self {
            Location::Hallway(pods, _) => (pods.pop(), 0),
            Location::Room(pods, _, _) => {
                let steps = 3 - pods.len();

                let popped = pods.pop();
                println!("Popped: {:?}", popped);
                println!("After pop: {}", &self);

                (popped, steps)
            }
        }
    }

    pub fn peek(&self) -> Option<&Amphipod> {
        match self {
            Location::Hallway(pods, _) => pods.last(),
            Location::Room(pods, _, _) => pods.last(),
        }
    }

    pub fn bottom_is_correct(&self) -> bool {
        match self {
            Location::Hallway(_, _) => false,
            Location::Room(pods, target, _) => {
                if let Some(bottom) = pods.first() {
                    *bottom == *target
                } else {
                    false
                }
            }
        }
    }

    pub fn current_occupancy_is_correct(&self) -> bool {
        match self {
            Location::Hallway(_, _) => false,
            Location::Room(pods, target, _) => {
                if let Some(top) = pods.last() {
                    self.bottom_is_correct() && *top == *target
                } else {
                    // todo: should be redundant
                    self.bottom_is_correct()
                }
            }
        }
    }

    pub fn has_movable(&self) -> bool {
        if let Some(pod) = self.peek() {
            // todo: double-op but should be correct
            !self.current_occupancy_is_correct() && pod.is_movable()
        } else {
            false
        }
    }




    /*
    pub fn move_pod(&mut self, target: &mut Location) {

    if !self.has_movable() {
    panic!("Should not happen")
    }

    if !target.available_spaces() == 0 {
    panic!("Cannot move into full space")
    }

    let ((pod, steps), from) = match &self {
    Location::Hallway(pods, position) => (self.pop(), position),
    Location::Room(pods, _, position) => (self.pop(), position),
    };

    let pod = pod.expect("Already checked");
    }

    */
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Amphipod {
    Amber(Vec<usize>),
    Bronze(Vec<usize>),
    Copper(Vec<usize>),
    Dessert(Vec<usize>),
}

impl Amphipod {
    pub fn cost(&self) -> usize {

        match self {
            Amphipod::Amber(moves) => (*moves).iter().sum::<usize>(),
            Amphipod::Bronze(moves) => 10*(*moves).iter().sum::<usize>(),
            Amphipod::Copper(moves) => 100*(*moves).iter().sum::<usize>(),
            Amphipod::Dessert(moves) => 1000*(*moves).iter().sum::<usize>(),
        }
    }

    pub fn is_movable(&self) -> bool {

        let can_move = |moves: &[usize]| moves.len() < 2;

        match self {
            Amphipod::Amber(moves) => can_move(moves),
            Amphipod::Bronze(moves) => can_move(moves),
            Amphipod::Copper(moves) => can_move(moves),
            Amphipod::Dessert(moves) => can_move(moves),
        }
    }

    pub fn has_moved(&self) -> bool {

        match self {
            Amphipod::Amber(moves) => !moves.is_empty(),
            Amphipod::Bronze(moves) => !moves.is_empty(),
            Amphipod::Copper(moves) => !moves.is_empty(),
            Amphipod::Dessert(moves) => !moves.is_empty(),
        }
    }

    pub fn add_steps(&mut self, steps: usize) {
        match self {
            Amphipod::Amber(moves) => moves.push(steps),
            Amphipod::Bronze(moves) => moves.push(steps),
            Amphipod::Copper(moves) => moves.push(steps),
            Amphipod::Dessert(moves) => moves.push(steps),
        }
    }
}

impl FromStr for Amphipod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::Amber(vec![])),
            "B" => Ok(Amphipod::Bronze(vec![])),
            "C" => Ok(Amphipod::Copper(vec![])),
            "D" => Ok(Amphipod::Dessert(vec![])),
            _ => Err(())
        }
    }
}

impl Display for Amphipod {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Amphipod::Amber(moves) => write!(f, "Amber {:?}", moves),
            Amphipod::Bronze(moves) => write!(f, "Bronze {:?}", moves),
            Amphipod::Copper(moves) => write!(f, "Copper {:?}", moves),
            Amphipod::Dessert(moves) => write!(f, "Dessert {:?}", moves)
        }
    }
}