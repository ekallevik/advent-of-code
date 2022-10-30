use crate::utils::get_comma_seperated_input;

pub fn solve_1(filename: &str) -> String {
    let input: Vec<isize> = get_comma_seperated_input(filename);
    let mut consumption = i64::MAX;

    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();

    for position in min..=max {
        let sum: i64 = input
            .iter()
            .map(|&crab| ((position - crab) as i64).abs())
            .sum();
        consumption = i64::min(sum as i64, consumption);
        println!("Pos: {:?} consumption: {:?}", position, consumption);
    }

    consumption.to_string()
}

fn sum_to_n(n: i64) -> i64 {
    n * (n + 1) / 2
}

pub fn solve_2(filename: &str) -> String {
    let input: Vec<isize> = get_comma_seperated_input(filename);
    let mut consumption = i64::MAX;

    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();

    for position in min..=max {
        let sum: i64 = input
            .iter()
            .map(|&crab| {
                let diff = ((position - crab) as i32).abs();
                sum_to_n(diff as i64)
            })
            .sum();
        consumption = i64::min(sum as i64, consumption);
    }

    consumption.to_string()
}
