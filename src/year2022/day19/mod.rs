use std::collections::{HashMap, VecDeque};
use std::f32::consts::E;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{bail, Result};
use scan_fmt::scan_fmt;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;
use crate::utils::{get_input, get_input_string};

struct Blueprint {
    id: usize,
    ore: Material,
    clay: Material,
    obsidian: Material,
    geode: Material,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let format = "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.";
        let result = scan_fmt!(s, format, usize, usize, usize, usize, usize, usize, usize);

        if let Ok((id, ore_robot_ore, clay_robot_ore, obs_robot_ore, obs_robot_clay, geode_robot_ore, geode_robot_obs)) = result {
            let ore_requirements = Material {
                ore: ore_robot_ore,
                clay: 0,
                obsidian: 0,
            };

            let clay_requirements = Material {
                ore: clay_robot_ore,
                clay: 0,
                obsidian: 0,
            };

            let obsidian_requirements = Material {
                ore: obs_robot_ore,
                clay: obs_robot_clay,
                obsidian: 0,
            };

            let geode_requirements = Material {
                ore: geode_robot_ore,
                clay: 0,
                obsidian: geode_robot_obs,
            };

            let blueprint = Blueprint {
                id,
                ore: ore_requirements,
                clay: clay_requirements,
                obsidian: obsidian_requirements,
                geode: geode_requirements,
            };

            Ok(blueprint)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug, EnumIter, Eq, PartialEq, Hash)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug)]
struct Material {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ore={}, clay={}, obsidian={}", self.ore, self.clay, self.obsidian)
    }
}

impl Material {
    fn is_larger(&self, other: &Material) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }

    fn subtract(&self, other: &Material) -> Material {
        Material {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
        }
    }
}


pub fn solve_1(filename: &str) -> Result<String> {
    let blueprints: Vec<Blueprint> = get_input(filename);

    let mut quality = vec![];

    for blueprint in blueprints {
        let mut robots: HashMap<Robot, usize> = Robot::iter().map(|r| (r, 0)).collect();
        robots.entry(Robot::Ore).and_modify(|value| *value += 1).or_insert(1);

        let inventory = Material {
            ore: 0,
            clay: 0,
            obsidian: 0,
        };

        let blueprint_quality = search(&blueprint, robots, &inventory, 0, 0, 0);
        println!("Blueprint {} => {}", blueprint.id, blueprint_quality);
        quality.push(blueprint_quality * blueprint.id);
    };

    let mut total_quality: usize = quality.iter().sum();

    Ok(total_quality.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let mut total_size = 0;

    Ok(total_size.to_string())
}

fn search(
    blueprint: &Blueprint,
    robots: HashMap<Robot, usize>,
    inventory: &Material,
    minute: isize,
    current_score: usize,
    best_score: usize,
) -> usize {

    //println!("- inventory: {}", inventory);
    //println!("- robots: {:?}", robots);

    if minute == 24 {
        return current_score;
    }

    let mut estimated_score = current_score;
    let mut geode_production = robots[&Robot::Geode];
    let mut afford_geode = inventory.is_larger(&blueprint.geode);

    for _ in minute..=24 {
        if afford_geode {
            geode_production += 1;
        }

        afford_geode = !afford_geode;

        estimated_score += geode_production;
    }
    //println!("{:>2} - score={:>2}, best={:>2}, estimate={:>2}", minute, current_score, best_score, estimated_score);

    if estimated_score <= best_score {
        //print!("️⚠️ Pruned! ⚠️");
        return current_score;
    }

    let mut next_score = current_score;
    let mut next_inventory = inventory.clone();

    for (robot, value) in &robots {
        match robot {
            Robot::Ore => next_inventory.ore += value,
            Robot::Clay => next_inventory.clay += value,
            Robot::Obsidian => next_inventory.obsidian += value,
            Robot::Geode => next_score += value,
        }
    }

    // generate new states
    let mut candidates = VecDeque::new();
    let mut can_afford_geode = false;

    if inventory.is_larger(&blueprint.ore) && minute != 23 {
        let next_material = next_inventory.subtract(&blueprint.ore);

        let mut next_robots = robots.clone();
        next_robots.entry(Robot::Ore).and_modify(|value| *value += 1).or_insert(1);

        candidates.push_front((next_material, next_robots));
    }

    if inventory.is_larger(&blueprint.clay) && minute != 23 {
        let next_material = next_inventory.subtract(&blueprint.clay);

        let mut next_robots = robots.clone();

        let mut v = next_robots.get_mut(&Robot::Clay).unwrap();
        *v += 1;

        candidates.push_front((next_material, next_robots));
    }

    if inventory.is_larger(&blueprint.obsidian) && minute != 23 {
        let next_material = next_inventory.subtract(&blueprint.obsidian);
        let mut next_robots = robots.clone();

        let mut v = next_robots.get_mut(&Robot::Obsidian).unwrap();
        *v += 1;

        candidates.push_front((next_material, next_robots));
    }

    if inventory.is_larger(&blueprint.geode) {
        let next_material = next_inventory.subtract(&blueprint.geode);

        let mut next_robots = robots.clone();
        next_robots.entry(Robot::Geode).and_modify(|value| *value += 1).or_insert(1);

        afford_geode = true;

        candidates.push_front((next_material, next_robots));
    }

    if !can_afford_geode {
        let next_material = next_inventory.clone();
        let mut next_robots = robots.clone();
        candidates.push_front((next_material, next_robots));
    }

    let mut best_branch = best_score;

    // iterate over states
    /*
    for (i, (candidate_inventory, candidate_robots)) in candidates.iter().enumerate() {
        println!("- Candidate {i} -> {} - {:?}", candidate_inventory, candidate_robots);
    }
    println!();
     */

    for (candidate_inventory, candidate_robots) in candidates {
        let candidate_score = search(blueprint, candidate_robots, &candidate_inventory, minute + 1, next_score, best_branch);

        if candidate_score > best_branch {
            println!("Found new best! Previous: {best_branch}, Next: {candidate_score}");
            best_branch = candidate_score;
        }
    };

    best_branch
}


