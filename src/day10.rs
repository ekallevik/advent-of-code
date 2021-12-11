use paris::{info, warn};


use crate::utils::{get_input_array};

pub fn solve_1(filename: String) -> String {
    let input = get_input_array::<char>(filename);

    let opening_def = vec!['(', '[', '{', '<'];
    let closing_def = vec![')', ']', '}', '>'];

    let mut corrupted = vec![];

    for (j, line) in input.iter().enumerate() {
        info!("\nLine {} - {:?}", j, line);

        let length = line.len();

        let mut openings = vec![];
        let mut current_opening: Option<char> = None;

        for (i, &c) in line.iter().enumerate() {
            if opening_def.contains(&c) {
                match current_opening {
                    None => {}
                    Some(value) => openings.push(value),
                }
                current_opening = Some(c);
            } else if closing_def.contains(&c) {
                let comp = match c {
                    ')' => current_opening == Some('('),
                    ']' => current_opening == Some('['),
                    '}' => current_opening == Some('{'),
                    '>' => current_opening == Some('<'),
                    _ => panic!("should not happen"),
                };

                if comp {
                    if i == length - 1 {
                        break;
                    }

                    current_opening = openings.pop()
                } else {
                    corrupted.push(c);
                    break;
                }
            } else {
                panic!("SHoud neve asd");
            }
        }
    }

    warn!("Corrupted: {:?}", corrupted);

    corrupted
        .iter()
        .map(|&c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum::<u64>()
        .to_string()
}

pub fn solve_2(filename: String) -> String {
    let input = get_input_array::<char>(filename);

    let opening_def = vec!['(', '[', '{', '<'];
    let closing_def = vec![')', ']', '}', '>'];

    let mut scores = vec![];
    let mut res = Vec::new();

    for (j, line) in input.iter().enumerate() {
        let length = line.len();

        let mut openings = vec![];
        let mut current_opening: Option<char> = None;

        let mut current_opening_is_added = true;
        let mut is_corrupted = false;

        for (i, &c) in line.iter().enumerate() {
            if opening_def.contains(&c) {
                match current_opening {
                    None => {}
                    Some(value) => openings.push(value),
                }
                current_opening_is_added = false;
                current_opening = Some(c);
            } else if closing_def.contains(&c) {
                let comp = match c {
                    ')' => current_opening == Some('('),
                    ']' => current_opening == Some('['),
                    '}' => current_opening == Some('{'),
                    '>' => current_opening == Some('<'),
                    _ => panic!("should not happen"),
                };

                if comp {
                    current_opening_is_added = true;

                    // if last
                    if i == length - 1 {
                        break;
                    }

                    current_opening = openings.pop()
                } else {
                    is_corrupted = true;
                    break;
                }
            } else {
                panic!("SHoud neve asd");
            }
        }

        if !is_corrupted {
            if !current_opening_is_added {
                match current_opening {
                    None => {}
                    Some(value) => openings.push(value),
                }
            }

            let mut score: i64 = 0;

            openings.reverse();

            for opening in openings {
                score *= 5;
                score += match opening {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("should not happen"),
                }
            }
            println!("Line {} has score: {}", j, score);
            res.push(j);
            scores.push(score);
        }
    }

    scores.sort_unstable();

    println!("{:?}", scores);
    let index = scores.len() / 2;
    scores[index].to_string()
}
