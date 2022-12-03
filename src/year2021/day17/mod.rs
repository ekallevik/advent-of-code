use anyhow::Result;
use std::cmp::max;
use std::ops::RangeInclusive;

fn parse_line(line: &str) -> Option<RangeInclusive<isize>> {
    let (_, content) = line.split_once('=')?;
    let (start, end) = content.split_once("..")?;
    Some(start.parse::<isize>().ok()?..=end.parse::<isize>().ok()?)
}

fn parse_input(filename: &str) -> Option<(RangeInclusive<isize>, RangeInclusive<isize>)> {
    let input = std::fs::read_to_string(filename).expect("file not found!");

    let (_, area) = input.split_once(": ")?;
    let (x, y) = area.split_once(", ")?;

    Some((parse_line(x)?, parse_line(y)?))
}


pub fn solve_1(filename: &str) -> Result<String> {
    let (target_x, target_y) = parse_input(filename).unwrap();

    //let mut velocities = HashSet::new();
    let mut max_initial_velocity = isize::MIN;


    for v_x_0 in calculate_v_x_range(&target_x) {

        for steps in (1..750).rev() {

            let v_y_min = max(max_initial_velocity, find_v_y_min(target_y.start(), steps));
            let v_y_max = find_v_y_max(target_y.end(), steps);

            if v_y_min >= v_y_max {
                continue
            }

            let vel_y_range = (v_y_min..v_y_max+1).rev();
            println!("v_x_0={}, steps={}, y_range={:?}", v_x_0, steps, vel_y_range);

            for v_y_0 in vel_y_range {

                /*
                let pos_x = calculate_pos(v_x_0, steps);
                let pos_y = calculate_pos_y(v_y_0, steps);

                if in_target_area(pos_x, pos_y, &target_x, &target_y) {

                }
                 */

                if hits_target(&target_x, &target_y, v_x_0, steps, v_y_0) {

                    max_initial_velocity = max(max_initial_velocity, v_y_0);

                    /*
                    velocities.insert((v_x_0, v_y_0));
                    let mut is_correct = String::new();
                    println!("Answer: ({}, {})\n", v_x_0, v_y_0);
                    println!("Is it correct?");
                    std::io::stdin().read_line(&mut is_correct).unwrap();

                     */
                    break;
                }
                /*
                                let (pos_x, pos_y) = step(v_x_0, v_y_0, steps);

                                if pos_x > *target_x.end() || pos_y < *target_y.start() {
                                    missed_target = true;
                                    break;
                                }

                 */
            }
        }
    }

    //println!("{:?}", cu);

    //let result = velocities.into_iter().max_by_key(|v| v.1).unwrap();
    Ok((sum_to_n(max_initial_velocity)).to_string())
}

/*
fn solve(target_x: RangeInclusive<isize>, target_y: RangeInclusive<isize>) -> usize {

    // 2753 too low

    let mut velocities = HashSet::new();

    let end = target_x.start()*2_isize;
    for v_x_0 in calculate_v_x_range(&target_x) {

        for steps in (1..end).rev() {

            //let v_y_min = find_v_y_min(target_y.start(), steps as usize);
            //let v_y_max = find_v_y_max(target_y.end(), steps as usize);

            //let vel_y_range = (v_y_min..=v_y_max);
            let vel_y_range = find_y_range(&target_y);
            println!("v_x_0={}, steps={}, y_range={:?}", v_x_0, steps, vel_y_range);


            for v_y_0 in vel_y_range {

                if velocities.contains(&(v_x_0, v_y_0)) {
                    continue
                }

                /*
                let pos_x = calculate_pos(v_x_0, steps);
                let pos_y = calculate_pos_y(v_y_0, steps);

                if in_target_area(pos_x, pos_y, &target_x, &target_y) {

                }
                 */

                if hits_target(&target_x, &target_y, v_x_0, steps as usize, v_y_0) {
                    velocities.insert((v_x_0, v_y_0));
                }
                /*
                                let (pos_x, pos_y) = step(v_x_0, v_y_0, steps);

                                if pos_x > *target_x.end() || pos_y < *target_y.start() {
                                    missed_target = true;
                                    break;
                                }

                 */
            }
        }
    }

    println!("{:?}", velocities);

    velocities.len()
}


fn step(v_x_0: isize, v_y_0: isize, steps: usize) -> (isize, isize) {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut v_x = v_x_0;
    let mut v_y = v_y_0;

    for _ in 0..steps {
        //pos_x += v_x;
        //v_x = max(0, v_x - 1);


        pos_y += v_y;
        v_y -= 1;
    }

    let pos_x = sum_to_n(min(v_x_0, steps as isize));
    //let pos_y = sum_to_n();


    (pos_x, pos_y)
}
*/

fn hits_target(target_x: &RangeInclusive<isize>, target_y: &RangeInclusive<isize>, v_x_0: isize, max_steps: usize, v_y_0: isize) -> bool {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut v_x = v_x_0;
    let mut v_y = v_y_0;

    let mut has_hit_target = false;

    for _ in 0..max_steps {
        pos_x += v_x;
        pos_y += v_y;
        v_x = max(0, v_x - 1);
        v_y -= 1;

        if target_x.contains(&pos_x) && target_y.contains(&pos_y) {
            //println!("Found possiblity: ({}, {})", v_x_0, v_y_0);
            has_hit_target = true;
            continue;
        }
    }

    has_hit_target
}

/*
fn in_target_area(pos_x: isize, pos_y: isize, target_x: &RangeInclusive<isize>, target_y: &RangeInclusive<isize>) -> bool {
    target_x.contains(&pos_x) && target_y.contains(&pos_y)
}
*/

fn sum_to_n(n: isize) -> isize {
    n * (n + 1) / 2
}

fn calculate_v_x_range(target: &RangeInclusive<isize>) -> RangeInclusive<isize> {
    let mut distance = 0;
    let mut v_x_min = 0;
    while distance < *target.start() {
        v_x_min += 1;
        distance = sum_to_n(v_x_min);
    }

    let v_x_max = *target.end()+5;

    v_x_min..=v_x_max
}

/*
fn calculate_pos(v_0: isize, steps: usize) -> isize {
    /*
    v_x_n = min(v_x_(n-1) - 1, 0)

    pos_x_3 = pos_x_2 + v_x_2 = (pos_x_1 + v_x_1) + v_x_2 = ((pos_x_0 + v_x_0) + v_x_1) + v_x_2
    pos_x_3 = pos_0 + v_x_0 + v_x_1 + v_x_2 = sum_2 v_x_n
    pos_x_3 = v_x_0 + (v_x_0 - 1) + (v_x_0 - 2)

    pos_x_n = pos_x_(n-1) + v_x_(n-1)
    k = min(n, v_x_0)
    pos_x_n = k*v_x_0 - sum_to_n(k-1)
     */

    let n = min(steps as isize, v_0);
    n * v_0 - sum_to_n((n - 1) as isize)
}

fn calculate_pos_y(v_y_0: isize, steps: usize) -> isize {
    /*
     pos_y_n = pos_y_n+ - pos_y_n-

     pos_y_n+ = pos_x_n
     steps_up = v_y_0
     pos_y_n- = sum_to_n(steps-steps_up)
     */

    let pos = calculate_pos(v_y_0, steps);
    let neg_steps = max(0, steps as isize - v_y_0-1);
    let neg = sum_to_n(neg_steps);

    pos - neg
}

pub fn solver() {
    let (x, y) = include_str!("real.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap();
    let (x, y) = (x.split_once("..").unwrap(), y.split_once("..").unwrap());
    let target: (_, i32, _, _) = (
        x.0.parse().unwrap(),
        y.0.parse().unwrap(),
        x.1.parse().unwrap(),
        y.1.parse().unwrap(),
    );

    println!(
        "{}",
        (1..=target.2)
            .flat_map(|vx| {
                let range = target.1.abs();
                (-range..=range).filter(move |&vy| fire(target, (vx, vy)))
            })
            .count()
    );
}


fn fire(target: (i32, i32, i32, i32), mut v: (i32, i32)) -> bool {
    let mut p = (0, 0);
    if v.1 > 1 {
        for _ in 0..v.1 * 2 + 1 {
            p.0 += v.0;
            v.0 -= 1;
            if p.0 > target.2 {
                return false;
            } else if v.0 == 0 {
                break;
            }
        }
        v.1 = -v.1 - 1;
    }
    for (x, y, vx, _) in path(p, v) {
        if vx == 0 && x < target.0 || x > target.2 || y < target.1 {
            return false;
        } else if x >= target.0 && x <= target.2 && y >= target.1 && y <= target.3 {
            return true;
        }
    }
    unreachable!();
}




fn path(p: (i32, i32), v: (i32, i32)) -> impl Iterator<Item = (i32, i32, i32, i32)> {
    std::iter::successors(Some((p.0, p.1, v.0, v.1)), |p| {
        Some((p.0 + p.2, p.1 + p.3, (p.2 - 1).max(0), p.3 - 1))
    })
}

 */


// todo: combine
fn find_v_y_max(highest_point: &isize, steps: usize) -> isize {

    // up - down <= high
    // up - (steps-up) <=  high
    // 2up - steps <= high
    // up <= (high+steps)/2
    // todo: check if y is within target


    let t = highest_point + steps as isize;
    t / 2 + 3
}

fn find_v_y_min(lowest_point: &isize, steps: usize) -> isize {



    // down >= low
    // todo: check if y is within target

    let target = -(*lowest_point as isize);
    //let vel = sum_to_n(target) - sum_to_n(target-steps as isize);

    if steps == 1 {
        *lowest_point
    } else {

        - (target / steps as isize)-600

        // t = 10
        // s = 2
        // r = 4 5

        // s = 3
        // r = 2 3 4

        // s = 4
        // r = 1 2 3 4
    }

}

/*

fn find_y_range(target_y: &RangeInclusive<isize>) -> RangeInclusive<isize> {
    let v_y_min = -target_y.start().abs();
    let v_y_max = target_y.end().abs()+20;
    v_y_min..=v_y_max
}

 */

pub fn solve_2(filename: &str) -> Result<String> {
    let (_target_x, _target_y) = parse_input(filename).unwrap();

    //solver();
    //"asd.".to_string()
    // 3528

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::year2021::day17::{calculate_v_x_range, find_v_y_max, find_v_y_min, hits_target};

    #[test]
    fn test_create_v_x_range() {
        let target = 20..=30;
        let actual = calculate_v_x_range(&target);

        let expected = 3..=10;

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_find_v_y_max() {
        let target_high = 3;
        let actual = find_v_y_max(&target_high, 4);

        let expected = 3;

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_find_v_y_max_2() {
        let target_high = -5;
        let actual = find_v_y_max(&target_high, 7);

        let expected = 2;

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_find_v_y_max_3() {
        let target_high = -5;
        let actual = find_v_y_max(&target_high, 9);

        let expected = 3;

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_find_v_y_max_5() {
        let target_high = -5;
        let actual = find_v_y_max(&target_high, 4);

        let expected = 0;

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_hits_target() {
        let target_x = 20..=30;
        let target_y = -10..=-5;

        let velocities = vec![
            (7, 2),
            (6, 3),
            (9, 0),
            (6, 9),
        ];

        for (v_x_0, v_y_0) in velocities {
            let actual = hits_target(&target_x, &target_y, v_x_0, 10, v_y_0);
            assert!(actual)
        }
    }

    #[test]
    fn test_does_not_hit_target() {
        let target_x = 20..=30;
        let target_y = -10..=-5;

        let v_x_0 = 17;
        let v_y_0 = -4;

        let actual = hits_target(&target_x, &target_y, v_x_0, 20, v_y_0);
        assert!(!actual)
    }

    #[test]
    fn test_find_min_y_real() {
        let parameters = vec![
            (1, -10..=-5, -10),
            (2, -10..=-5, -4),
        ];

        for (steps, target_y, expected) in parameters {
            let actual = find_v_y_min(target_y.start(), steps);
            println!("steps={}, actual={}, expected={}", steps, actual, expected);
            assert_eq!(actual, expected)
        }
    }

}

/*
 up - down <= high
 up - (steps-up) <=  high
 2up - steps <= high
 up <= (high+steps)/2
 up = floor((high+steps)/2)

 high = -5
 steps = 7
 calc = floor((-5+7)/2) = 3





 3, 0

 2, 3
 1, 5
 0, 6
-1, 5
-2, 3
-3, 0

 */