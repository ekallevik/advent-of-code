use crate::utils::{get_input, get_input_array};

pub fn solve_1(filename: &str) -> String {
    let input = get_input_array(filename);

    let size = input.len();
    let length = input.first().unwrap().len();

    let ones = count_number_of_ones(input);
    let gamma = calculate_gamma(ones, size, length);

    let max_value = 2i64.pow(length as u32) - 1;
    let epsilon = max_value - gamma;

    (epsilon * gamma).to_string()
}

pub fn solve_2(filename: &str) -> String {
    let input_a: Vec<Vec<u8>> = get_input_array(filename);
    let input2 = get_input(filename);
    let input: Vec<String> = get_input(filename);

    let length = input.first().unwrap().len();

    let mut oxygen_list: Vec<Vec<char>> = convert_input(input);
    let mut co2_list: Vec<Vec<char>> = convert_input(input2);

    let mut oxygen = 0;
    let mut co2 = 0;

    let ox_list = input_a;
    let size = ox_list.first().unwrap().len();
    let ox = filter_inputs(ox_list, length, size);

    for i in 0..length {
        let most_common_bit = find_most_common_bit_from_char(&oxygen_list, i);
        oxygen_list = oxygen_list
            .into_iter()
            .filter(|elem| elem[i] == most_common_bit)
            .collect();

        if oxygen_list.len() == 1 {
            for i in 0..length {
                if oxygen_list[0][i] == '1' {
                    oxygen += 2i64.pow((length - i - 1) as u32);
                }
            }
        }
    }

    for i in 0..length {
        let most_common_bit = find_most_common_bit_from_char(&co2_list, i);
        co2_list = co2_list
            .into_iter()
            .filter(|elem| elem[i] != most_common_bit)
            .collect();

        if co2_list.len() == 1 {
            for i in 0..length {
                if co2_list[0][i] == '1' {
                    co2 += 2i64.pow((length - i - 1) as u32);
                }
            }
            println!(
                "\n####\nTarget: {:?} with value={:?}\n####\n",
                oxygen_list, oxygen
            );
        }
    }

    if ox != oxygen {
        panic!("Oxygen: {}, ox: {}", oxygen, ox);
    }

    (oxygen * co2).to_string()
}

fn count_ones(lines: &Vec<Vec<u8>>, index: usize) -> usize {
    lines.iter().filter(|&line| line[index] == 1).count()
}

fn count_ones2(lines: Vec<Vec<u8>>, index: usize) -> usize {
    lines.iter().filter(|&line| line[index] == 1).count()
}

fn get_most_common_bit(lines: &Vec<Vec<u8>>, index: usize, size: usize) -> usize {
    let ones = count_ones(lines, index);
    if 2 * ones >= size {
        1
    } else {
        0
    }
}

fn get_most_common_bit2(lines: Vec<Vec<u8>>, index: usize, size: usize) -> usize {
    let ones = count_ones2(lines, index);
    if 2 * ones >= size {
        1
    } else {
        0
    }
}

/// For each index, count the number of ones, across every line.
fn count_number_of_ones(lines: Vec<Vec<u8>>) -> Vec<usize> {
    let number_of_lines = lines.first().unwrap().len();

    (0..number_of_lines)
        .map(|i| count_ones(&lines, i))
        .collect()
}

/// Generation of gamma as a binary number:
/// For every index where 1  the frequency of 1's makes up at least half the size,
/// retain 1, otherwise retain 0.
///
/// Create a binary number based on these 1's and 0's, and then convert this binary number to decimal
///
/// These two steps are implemented as one step.
fn calculate_gamma(frequencies: Vec<usize>, size: usize, length: usize) -> i64 {
    frequencies
        .iter()
        .enumerate()
        .map(|(i, freq)| {
            if 2 * freq >= size {
                2i64.pow((length - i - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

fn calc(frequency: u8, size: usize, length: usize, index: usize) -> i64 {
    if 2 * frequency >= size as u8 {
        2i64.pow((length - index - 1) as u32)
    } else {
        0
    }
}

fn calculate_gamma2(frequencies: Vec<u8>, size: usize, length: usize) -> i64 {
    let res = frequencies
        .iter()
        .enumerate()
        .map(|(i, &freq)| calc(freq, size, length, i))
        .sum();

    println!("res: {}", res);
    res
}

fn find_most_common_bit_from_char(lines: &Vec<Vec<char>>, i: usize) -> char {
    let zeros = lines.iter().filter(|&x| x[i] == '0').count();
    let ones = lines.iter().filter(|&x| x[i] == '1').count();

    if zeros > ones {
        '0'
    } else {
        '1'
    }
}

fn convert_input(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .into_iter()
        .map(|elem| elem.chars().collect::<Vec<_>>())
        .collect()
}

fn filter_inputs(input_a: Vec<Vec<u8>>, length: usize, size: usize) -> i64 {
    let mut value = 0;
    let mut input = input_a;

    for i in 0..length {
        let copy = input
            .iter()
            .map(|x| x.iter().clone().copied().collect())
            .collect();
        let most_common_bit = get_most_common_bit2(copy, i, size);
        input = input
            .into_iter()
            .filter(|elem| elem[i] == most_common_bit as u8)
            .collect();

        if input.len() == 1 {
            let target: Vec<u8> = input[0].iter().clone().copied().collect();
            print!("\n####\nTarget OX: {:?}", target);
            value = calculate_gamma2(target, 1, length);
            println!(" with value={:?}\n####\n", value);

            break;
        }
    }

    value
}
