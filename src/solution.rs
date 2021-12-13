use crate::PuzzlePart;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Solution {
    day: u32,
    first_test: Option<String>,
    first_real: Option<String>,
    second_test: Option<String>,
    second_real: Option<String>,
}

impl Solution {
    pub fn load_or_create(day: u32) -> Solution {
        let filename = format!("src/solution{}.json", day);
        let path = std::path::Path::new(&filename);

        if path.exists() {
            Solution::load(path)
        } else {
            let solution: Solution = Solution {
                day,
                first_test: None,
                first_real: None,
                second_test: None,
                second_real: None,
            };
            solution.save();
            solution
        }
    }

    pub fn save(&self) {
        let filename = format!("src/solution{:02}.json", self.day);
        let path = std::path::Path::new(&filename);
        let data = serde_json::to_string(&self).unwrap();
        fs::write(path, data).expect("Unable to write file");
    }

    pub fn load(path: &Path) -> Solution {
        let data = fs::read_to_string(path).expect("Unable to load file");
        serde_json::from_str(data.as_str()).unwrap()
    }

    pub fn verify_or_update(&mut self, part: PuzzlePart, result: String) -> bool {
        let answer = match part {
            PuzzlePart::FirstTest => &mut self.first_test,
            PuzzlePart::FirstReal => &mut self.first_real,
            PuzzlePart::SecondTest => &mut self.second_test,
            PuzzlePart::SecondReal => &mut self.second_real,
        };

        let verified = match &answer {
            None => {
                let mut is_correct = String::new();
                println!("{}", result);
                println!("Is it correct for {:?}?... ", part);
                std::io::stdin().read_line(&mut is_correct).unwrap();
                if is_correct.trim() == "y" {
                    println!("correct");
                    *answer = Some(result);
                    true
                } else {
                    println!("incorrect");
                    false
                }
            }
            Some(value) => *value == result,
        };

        verified
    }
}
