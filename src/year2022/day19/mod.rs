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

        // todo: test also with ore
        let clay_quality = search(&blueprint, robots.clone(), &inventory, 1, 0, 0, Robot::Clay);
        let ore_quality = search(&blueprint, robots, &inventory, 1, 0, clay_quality, Robot::Ore);

        let blueprint_quality = clay_quality.max(ore_quality);
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
    target: Robot,
) -> usize {

    if minute == 24 {
        return current_score;
    }

    let mut best_branch = best_score;

    println!("\n{minute:>2} - score: {current_score:>2} -> {best_score:>2}, target: {target:?}");
    //println!("start robots     : {:?}", robots);
    //println!("start inventory  : {}", inventory);

    let target_cost = match target {
        Robot::Ore => &blueprint.ore,
        Robot::Clay => &blueprint.clay,
        Robot::Obsidian => &blueprint.obsidian,
        Robot::Geode => &blueprint.geode,
    };

    let can_afford_target = inventory.is_larger(&target_cost);

    /*
    println!("robots: {:?}", robots);
    println!("target: {target:?}");
    println!("inventory  : {}", inventory);
    println!("current_inv: {next_inventory}");
    println!("target_cost: {target_cost}");
    println!("affordable : {can_afford_target}");
     */

    let mut next_score = current_score + robots.get(&Robot::Geode).unwrap();
    let mut next_inventory = inventory.clone();
    next_inventory.ore += robots.get(&Robot::Ore).unwrap();
    next_inventory.clay += robots.get(&Robot::Clay).unwrap();
    next_inventory.obsidian += robots.get(&Robot::Obsidian).unwrap();

    let mut next_robots = robots.clone();

    if !can_afford_target {
        //println!("next inventory   : {}", next_inventory);
        //println!("Could not afford. Skipping...");
        // todo: update values and skip time steps...
        return search(blueprint, robots, &next_inventory, minute + 1, next_score, best_branch, target);
    }

    let mut robot_entry = next_robots.get_mut(&target).unwrap();
    *robot_entry += 1;

    let next_material = next_inventory.subtract(target_cost);
    //println!("next robots      : {:?}", next_robots);
    //println!("next inventory   : {next_material}");

    // todo: make sure if target
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
        print!("️⚠️ Pruned! ⚠️");
        return current_score;
    }



    // generate new states
    let mut candidates = VecDeque::new();
    candidates.push_front(Robot::Ore);
    candidates.push_front(Robot::Clay);
    candidates.push_front(Robot::Obsidian);
    candidates.push_front(Robot::Geode);

    for candidate in candidates {
        // todo: cleanup ref and clones
        let candidate_score = search(blueprint, next_robots.clone(), &next_material, minute + 1, next_score, best_branch, candidate);

        if candidate_score > best_branch {
            println!("{minute:>2} - Found new best! {best_branch} -> {candidate_score}");
            best_branch = candidate_score;
        }
    };

    best_branch
}


