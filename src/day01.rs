use crate::utils::{get_input, PuzzlePart};

pub fn solve(part: PuzzlePart) -> u64 {
    println!("Puzzle day 01 - {:?}", part);
    let input = get_input::<u64>("src/input01.txt");

    match part {
        PuzzlePart::Part1 => count_depth_increases(input),
        PuzzlePart::Part2 => count_sliding_depth_increases(input),
    }
}

fn count_depth_increases(depths: Vec<u64>) -> u64 {
    depths
        .windows(2)
        .map(|x| if x[0] < x[1] { 1 } else { 0 })
        .sum()
}

fn count_sliding_depth_increases(depths: Vec<u64>) -> u64 {
    depths
        .windows(4)
        .map(|x| {
            if x[0] < x[3] {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_depth_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = count_depth_increases(input);
        assert_eq!(result, 7)
    }

    #[test]
    fn test_count_sliding_depth_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = count_sliding_depth_increases(input);
        assert_eq!(result, 5)
    }
}
