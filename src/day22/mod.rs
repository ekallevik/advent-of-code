use crate::utils::get_input;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use crate::domain::Line;
use crate::domain::Cube;


type SimpleCube = (
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
);

// todo: improve part1 solution with superfluous inputs
// todo: use tuples
// todo: performance
fn parse_line(line: &str) -> Option<(bool, SimpleCube)> {
    let (active, rest) = line.split_once(" ")?;

    let active = active == "on";

    let cleaned = rest
        .split(",")
        .map(|part| part.split_once("=").unwrap().1)
        .collect::<Vec<&str>>();

    let cube: Vec<RangeInclusive<isize>> = cleaned
        .into_iter()
        .map(|part| part.split_once("..").unwrap())
        .map(|(a, b)| (a.parse::<isize>().unwrap()..=b.parse::<isize>().unwrap()))
        .collect();

    let unwrapped = cube;

    Some((
        active,
        (
            unwrapped[0].clone(),
            unwrapped[1].clone(),
            unwrapped[2].clone(),
        ),
    ))
}

fn filter_range(range: &RangeInclusive<isize>) -> bool {
    *range.start() <= 50 && *range.end() >= -50
}

fn parse_step(step: String) -> (bool, Cube) {
    let (active, rest) = step.split_once(" ").unwrap();
    let active = active == "on";

    let cleaned: Vec<&str> = rest
        .split(",")
        .map(|part| part.split_once("=").unwrap().1)
        .collect();

    let lines: Vec<Line> = cleaned
        .into_iter()
        .map(|part: &str| part.parse::<Line>().unwrap())
        .collect();

    (active, Cube(lines[0], lines[1], lines[2]))
}

fn parse_steps(filename: &str, core_only: bool) -> Vec<(bool, Cube)> {
    let input: Vec<String> = get_input(filename);

    let steps: Vec<(bool, Cube)> = input
        .into_iter()
        .map(parse_step)
        .collect::<Vec<(bool, Cube)>>();

    let core = Cube::new_symmetric(-50, 50);

    match core_only {
        false => steps,
        true => {
            let filtered = steps
                .into_iter()
                .filter(|(_, cube)| cube.overlaps(&core))
                .collect::<Vec<(bool, Cube)>>();

            for (_, c) in filtered.iter() {
                println!("{}", c);
            }

            println!("\n\n");

            filtered
        }
    }
}


fn parse_input(filename: &str, only_core: bool) -> Vec<(bool, SimpleCube)> {
    let input: Vec<String> = get_input(filename);

    let steps = input
        .into_iter()
        .map(|line| parse_line(&line).unwrap())
        .collect::<Vec<(bool, SimpleCube)>>();

    match only_core {
        false => steps,
        true => steps
            .into_iter()
            .filter(|(_, (x_range, y_range, z_range))| {
                filter_range(x_range) && filter_range(y_range) && filter_range(z_range)
            })
            .collect(),
    }
}

// todo: fix parsing
pub fn solve_1_old(filename: &str) -> String {
    let input = parse_input(filename, true);

    reboot_reactor(&input).to_string()
}

pub fn solve_1(filename: &str) -> String {
    let steps = parse_steps(filename, true);

    //"asd".to_string()
    reboot(&steps).to_string()
}

pub fn solve_2(filename: &str) -> String {
    let steps = parse_steps(filename, false);

    reboot(&steps).to_string()
}

fn reboot_reactor(steps: &[(bool, SimpleCube)]) -> usize {
    let mut reactor = HashSet::new();

    let mut step = 1;

    for (state, (x_range, y_range, z_range)) in steps {
        step += 1;
        for x in x_range.clone() {
            println!("- x={}", x);
            for y in y_range.clone() {
                for z in z_range.clone() {
                    match *state {
                        false => reactor.remove(&(x, y, z)),
                        true => reactor.insert((x, y, z)),
                    };
                }
            }
        }
    }

    reactor.len()
}

fn reboot(steps: &[(bool, Cube)]) -> usize {

    // todo: replace with hashset?
    // todo: reduce memory usage
    // todo: combine neighbors?
    let mut reactor: HashSet<Cube> = HashSet::new();

    let mut step = 0;
    let number_of_steps = steps.len();


    for (activate, step_cube) in steps {
        step += 1;
        println!("Applying step {} of {}", step, number_of_steps);

        // todo: move this one level up?
        if reactor.is_empty() && *activate {
            reactor.insert(*step_cube);
            continue;
        }

        let clone = reactor.clone();
        let affected: Vec<&Cube> = clone.iter().filter(|&r_cube| (*r_cube).overlaps(step_cube)).collect();

        let mut added_cubes = vec![];
        let mut removed_cubes = vec![];

        for affected_cube in affected {

            reactor.remove(affected_cube);
            removed_cubes.append(&mut affected_cube.get_cuboids().clone());

            if *activate {
                // todo: might be a bug here as well
                let inter = affected_cube.intersection(step_cube);

                if let Some(inter) = inter {
                    reactor.insert(inter.clone());

                    let a_diff = affected_cube.subtract(step_cube);
                    for a_d in a_diff {
                        reactor.insert(a_d.clone());
                    }

                    let s_diff = step_cube.subtract(&affected_cube);
                    for s_d in s_diff {
                        reactor.insert(s_d.clone());
                    }
                }
            } else {
                let updated = affected_cube.subtract(step_cube);

               // println!("\n#### - START\n");
                for u in updated {
                    //println!("{}\n", u);

                    added_cubes.append(&mut affected_cube.get_cuboids().clone());

                    // todo:
                    // why is

                    reactor.insert(u);
                }
                //println!("\n#### - END\n");
            }
        }

        let actual_removed = removed_cubes.iter().filter(|&rc| !added_cubes.contains(rc)).collect::<Vec<&(isize, isize, isize)>>();

        println!("Removed...");
        for ar in actual_removed {
            println!("{:?}", ar);
        }
    };


    //      474140
    //    85351966
    // 40758233331

    let mut cuboids: Vec<(isize, isize, isize)> = vec![];
    let mut cuboid_set = HashSet::new();
    for c in reactor.iter() {
        //println!("{}", c);
        let mut cb = c.get_cuboids();
        for cbs in cb.iter() {
            cuboid_set.insert(cbs.clone());
        }
        cuboids.append(&mut cb);

    }

    cuboids.sort_by_key(|&c| c.2);
    cuboids.sort_by_key(|&c| c.1);
    cuboids.sort_by_key(|&c| c.0);

    for (cx, cy, cz) in cuboid_set.clone() {
        //println!("{},{},{}", cx, cy, cz);
    }

    println!("Reactor length: {}", reactor.len());
    println!("CBS length: {}", cuboid_set.len());
    println!("C length: {}", cuboids.len());

    reactor.into_iter().map(|cube| cube.size()).sum()
}

