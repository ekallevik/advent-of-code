
use crate::utils::get_input;

pub fn solve_1(filename: String) -> String {
    let input = get_input(filename);
    count_depth_increases(input).to_string()
}

pub fn solve_2(filename: String) -> String {
    let input = get_input(filename);
    count_sliding_depth_increases(input).to_string()
}

fn count_depth_increases(depths: Vec<u64>) -> u64 {
    depths
        .windows(2)
        .map(|x| if x[0] < x[1] { 1 } else { 0 } )
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
