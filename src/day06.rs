use crate::utils::get_comma_seperated_input;

pub fn solve_1(filename: &str) -> String {
    solve(filename, 80)
}

pub fn solve_2(filename: &str) -> String {
    solve(filename, 256)
}

fn solve(filename: &str, days: u16) -> String {

    let input: Vec<u32> = get_comma_seperated_input(filename);

    let mut count = (0..=8)
        .map(|i| input.iter().filter(|&fish| *fish == i).count() as u64)
        .collect::<Vec<u64>>();

    for _ in 0..days {
        let number_of_zeroes = count[0];
        count = count[1..].to_vec();
        count[6] += number_of_zeroes;
        count.push(number_of_zeroes);
    }

    count.iter().sum::<u64>().to_string()
}
