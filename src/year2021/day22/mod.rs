use crate::utils::{breakpoint, get_input};
use std::collections::HashSet;
use crate::domain::Line;
use crate::domain::Cube;

// todo: removed unused

fn parse_step(step: String) -> (bool, Cube) {
    let (active, rest) = step.split_once(' ').unwrap();
    let active = active == "on";

    let lines: Vec<Line> = rest
        .split(',')
        .map(|part| part.split_once('=').unwrap().1)
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



pub fn solve_1(filename: &str) -> String {
    let steps = parse_steps(filename, true);

    reboot(steps).to_string()
}


pub fn solve_2(filename: &str) -> String {
    let steps = parse_steps(filename, false);

    reboot(steps).to_string()
}

/*
fn reboot_reactor(steps: &[(bool, SimpleCube)]) -> usize {
    let mut reactor = HashSet::new();

    for (state, (x_range, y_range, z_range)) in steps {
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

 */

fn calculate_size(reactor: &[Cube]) -> usize {
    reactor.iter().map(|cube| cube.size()).sum()
}

fn reboot(steps: Vec<(bool, Cube)>) -> usize {

    // todo: replace with vec??
    let mut reactor: Vec<Cube> = vec![];

    let mut step = 0;
    let number_of_steps = steps.len();

    for (activate, step_cube) in steps {
        step += 1;
        println!("Applying step {} of {}", step, number_of_steps);
        println!("Number of reactor cubes: {}", reactor.len());
        let r_size = calculate_size(&reactor);

        let (mut affected_cubes, mut new_reactor): (Vec<Cube>, Vec<Cube>) = reactor
            .into_iter()
            .partition(|reactor_cube| step_cube.overlaps(reactor_cube));

        {
            if calculate_size(&new_reactor) + calculate_size(&affected_cubes) != r_size {
                panic!("asda");
            }
            println!("Number of reactor cuboids: {}", r_size);
        }

        println!("Step cube: {}", step_cube);
        for a in affected_cubes.iter() {
            println!("Affects: {}", a);
        }


        breakpoint("Continue??");

        // todo: write more functional
        if activate {

            let mut additions = vec![step_cube];

            for affected in affected_cubes.iter() {
                let mut updated_sum = vec![];
                for sum_cube in additions {
                    // todo: test this properly
                    let components = sum_cube.subtract(affected);
                    updated_sum.extend(components);
                }
                additions = updated_sum;
            }

            new_reactor.append(&mut affected_cubes);
            new_reactor.append(&mut additions);

        } else {
            for affected in affected_cubes {
                let mut difference_cubes = affected.subtract(&step_cube);
                new_reactor.append(&mut difference_cubes);
            }
        }

        reactor = new_reactor;
    };

    println!("Completed all steps");



    let mut cuboids: HashSet<(isize, isize, isize)> = HashSet::new();
    for c in reactor.iter()  {
        let cb = c.get_cuboids();
        for c in cb {
            cuboids.insert(c);
        }
    }
    println!("Cuboids length: {}", cuboids.len());





    println!("Reactor length: {}", reactor.len());

    // todo: partion on isolated cubes
    // isolated cube => size
    // overlapping cubes => count

    reactor.into_iter().map(|cube| cube.size()).sum()
}

/*
pub type SimpleCube = (
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
);


// todo: improve part1 solution with superfluous inputs
// todo: use tuples
// todo: performance
pub fn parse_line(line: &str) -> Option<(bool, SimpleCube)> {
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



pub fn filter_range(range: &RangeInclusive<isize>) -> bool {
    *range.start() <= 50 && *range.end() >= -50
}
 */
