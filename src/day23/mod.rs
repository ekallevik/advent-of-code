use crate::day23::amphipod::Amphipod;
use crate::day23::location::Location;

mod location;
mod amphipod;

// todo: fix typo

/*
THE RULES OF THE GAME
1. each move cost energy
2. can only move into open space
3. can only move Manhattan-style
4. the Amphipods should be sorted A-D left to right
5. Amphipods will never stop just outside a room, but can pass through
6. Amphipods will only move into the room that it it's finally destination
    & it only contains Amphipods of the correct tyoe
7. Once moved into the hallway, it will stay there til it can move directly to destination
*/

pub fn solve_1(filename: &str) -> String {

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

    solve(locations, 0, 0).unwrap().to_string()
}

pub fn solve_2(filename: &str) -> String {
    filename.to_string()
}

fn solve(locations: Vec<Location>, iteration: usize, branch: usize) -> Option<usize> {

    let mut costs = vec![];
    let mut candidates: Vec<usize> = locations.iter().filter(|loc| loc.has_movable()).map(|loc| loc.position()).collect();

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

    costs.into_iter().filter_map(|cost| cost).min()
}

fn find_targets<'a>(locations: &'a Vec<Location>, position: usize, peek: &Amphipod) -> Vec<&'a Location> {

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

fn calculate_score(locations: &Vec<Location>) -> Option<usize> {

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
    print_locations(&locations);
    println!("\n### END ###\n");

    Some(score)
}

fn print_locations(locations: &Vec<Location>) {
    for candidate in locations.iter() {
        println!("{}", candidate)
    }
}

fn print_locations2(locations: &Vec<&Location>) {
    for candidate in locations.iter() {
        println!("{}", candidate)
    }
}

fn apply_move(mut locs: Vec<Location>, from: usize, to: usize, iteration: usize, branch: usize) -> Option<usize> {

    let from = locs.get_mut(from).unwrap();
    let from_pos = from.position() as isize;
    let (pod, up_steps) = from.pop();

    let to = locs.get_mut(to).unwrap();
    let sideways_steps = (to.position() as isize - from_pos).abs() as usize;

    let mut pod = pod.unwrap();
    let steps: usize = up_steps + sideways_steps;

    to.push_and_update(pod, steps);

    solve(locs, iteration+1, branch)
}
