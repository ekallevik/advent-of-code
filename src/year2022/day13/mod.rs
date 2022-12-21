use std::str::FromStr;
use anyhow::Result;
use itertools::Itertools;
use crate::utils::get_input_string;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Distress {
    Array(Vec<Distress>),
    Number(usize)
}

impl FromStr for Distress {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {

        let result = if let Ok(number) = s.parse::<usize>() {
            Distress::Number(number)
        } else if s.is_empty() {
            Distress::Array(vec![])
        }  else if !s.contains('[') {
            println!("s={s}");
            let arr = s
                .trim()
                .split(',')
                .into_iter()
                .map(|a| a.parse::<usize>().unwrap())
                .map(Distress::Number)
                .collect_vec();

            Distress::Array(arr)
        } else {
            println!();

            let content = s
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap();

            if let Ok(number) = content.parse::<usize>() {
                return Ok(Distress::Number(number))
            }

            let mut openings = 0;
            let mut closings = 0;
            let mut midpoint = 0;

            for (i, c) in content.chars().enumerate() {
                if c == '[' {
                    openings += 1;
                } else if c == ']' {
                    closings += 1;
                }

                if c == ',' && openings == closings {
                    midpoint = i;
                }
            }

            println!("s={s}, content={content}\nmidpoint={midpoint:?}, openings={openings}, closings={closings}");
            println!("start={}, end={}, openings={}", content.starts_with('['), content.ends_with(']'), openings == 1);

            if openings == 1 {
                println!("Found one array");

                if content.starts_with('[') && content.ends_with(']') {
                    println!("Enclosed arr");
                    let c = content
                        .strip_prefix('[')
                        .unwrap()
                        .strip_suffix(']')
                        .unwrap();
                    println!("c={c}");
                    let distress = c.parse().unwrap();
                    println!("value={distress:?}");
                    Distress::Array(vec![distress])
                } else if content.starts_with('[') {
                    println!("Leading arr");
                    let c = content
                        .strip_prefix('[')
                        .unwrap();

                    let (left, right) = c.split_once("],").unwrap();
                    println!("l={left}, r={right}");
                    let l = left.parse().unwrap();
                    let r = right.parse().unwrap();
                    let arr = vec![l, r];
                    Distress::Array(arr)
                } else {
                    println!("Trailing arr");
                    let c = content
                        .strip_suffix(']')
                        .unwrap();

                    let (left, right) = c.split_once(",[").unwrap();
                    let l = left.parse().unwrap();
                    let r = right.parse().unwrap();
                    let arr = vec![l, r];
                    Distress::Array(arr)
                }


            } else if openings > 0 {
                println!("Found pair");
                let (left, right_a) = content.split_at(midpoint);
                let right = right_a.strip_prefix(',').unwrap();
                println!("left={left}, right={right}\n");

                let l = left.parse().unwrap();
                let r = right.parse().unwrap();
                let arr = vec![l, r];
                Distress::Array(arr)
            } else {
                println!("Found else");
                let arr = content
                    .split(',')
                    .into_iter()
                    .map(|a| a.parse().unwrap())
                    .collect_vec();

                Distress::Array(arr)
            }


        };

        Ok(result)
    }
}

fn parse_input(filename: &str) {

    let input = get_input_string(filename);
    let lines = input.split_ascii_whitespace().collect_vec();

    for a in lines.windows(2) {
        let left: Distress = a[0].parse().unwrap();
        let right: Distress = a[1].parse().unwrap();

        println!("{left:?}");
        println!("{right:?}");
        println!();
    }

}

pub fn solve_1(filename: &str) -> Result<String> {

    parse_input(filename);


    todo!()
}


pub fn solve_2(filename: &str) -> Result<String> {
    parse_input(filename);

    todo!()
}