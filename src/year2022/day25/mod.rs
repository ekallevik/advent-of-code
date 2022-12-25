use std::ptr::null;
use anyhow::{bail, Result};
use crate::utils::{get_input, get_input_string};

pub fn solve_1(filename: &str) -> Result<String> {
    let lines: Vec<String> = get_input(filename);

    let sum: isize = lines.iter().map(|snafu| convert_from_snafu(snafu)).sum();
    let snafu = convert_to_snafu(sum);

    Ok(snafu)
}

fn convert_from_snafu(snafu: &str) -> isize {
    let mut decimal = 0;

    for (place, d) in snafu.chars().rev().enumerate() {
        decimal += convert_from_snafu_digit(d) * 5_isize.pow(place as u32);
    }

    decimal
}

fn convert_from_snafu_digit(d: char) -> isize {
    match d {
        '-' => -1,
        '=' => -2,
        v => v.to_digit(10).unwrap() as isize,
    }
}

fn convert_to_snafu_digit(digit: isize) -> Result<char> {
    match digit {
        -2 => Ok('='),
        -1 => Ok('-'),
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        _ => bail!("Digit not supported")
    }
}


fn convert_to_snafu(number: isize) -> String {
    let mut place = 1;

    while number > 5_isize.pow(place - 1) {
        place += 1;
    }

    let mut snafu = "".to_string();
    let mut current = number;

    for p in (1..place).rev() {

        let power = 5_isize.pow(p);

        let bound: isize = (0..p).into_iter().map(|p| 5_isize.pow(p)).sum();
        let bounds = (-2 * bound)..=(2 * bound);

        if bounds.contains(&(current - 2 * power)) {
            snafu += "2";
            current -= 2 * power;
        } else if bounds.contains(&(current - power)) {
            snafu += "1";
            current -= power;
        } else if current == 0 {
            snafu += "0";
        } else if bounds.contains(&(current + 2* power))  {
            snafu += "=";
            current += 2 * power;
        } else if bounds.contains(&(current + power))  {
            snafu += "-";
            current += power;
        } else if snafu != "" {
            snafu += "0";
        }
    }

    snafu.push(convert_to_snafu_digit(current).unwrap());

    snafu.to_string()
}

pub fn solve_2(filename: &str) -> Result<String> {
    println!("Have you collected all 49 ⭐️ fruit?");
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();

    Ok(answer)
}
