use crate::utils::get_partitioned_input;

fn parse_to_boolean(char: char) -> bool {
    match char {
        '#' => true,
        '.' => false,
        _ => panic!("Should not happen"),
    }
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(parse_to_boolean).collect()
}

fn parse_input(filename: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let (first, second) = get_partitioned_input(filename);

    let input = first.lines().flat_map(parse_line).collect();

    let output = second.lines().map(parse_line).collect();

    (input, output)
}

pub fn solve_1(filename: &str) -> String {
    let (codec, image) = parse_input(filename);

    let (first, new_default) = apply_codec(&codec, image, false);
    let (second, _) = apply_codec(&codec, first, new_default);

    count_light_pixels(&second).to_string()
}

pub fn solve_2(filename: &str) -> String {
    let (codec, image) = parse_input(filename);

    let mut image = image;
    let mut default = false;

    // todo: improve this
    for i in 0..50 {
        println!("Iteration: {}", i);
        let (updated_image, updated_default) = apply_codec(&codec, image, default);
        image = updated_image;
        default = updated_default;
    }

    count_light_pixels(&image).to_string()
}

fn count_light_pixels(image: &[Vec<bool>]) -> usize {
    image
        .iter()
        .map(|row| (*row).iter().filter(|&char| *char).count())
        .sum::<usize>()
}

fn apply_codec(codec: &[bool], image: Vec<Vec<bool>>, default: bool) -> (Vec<Vec<bool>>, bool) {
    let (extended_image, extend_size) = extend_image(image, default);

    let output: Vec<Vec<bool>> = (0..extend_size)
        .into_iter()
        .map(|i| create_updated_row(codec, default, &extended_image, extend_size, i))
        .collect();

    let new_default = if !default && *codec.first().unwrap() {
        true
    } else if default && !(*codec.last().unwrap()) {
        false
    } else {
        default
    };

    (output, new_default)
}

fn create_updated_row(
    codec: &[bool],
    default: bool,
    extended_image: &[Vec<bool>],
    extend_size: usize,
    i: usize,
) -> Vec<bool> {
    (0..extend_size)
        .into_iter()
        .map(|j| get_pixel_value(extended_image, i as isize, j as isize, default))
        .map(|value| codec[value])
        .collect()
}

fn get_pixel_value(image: &[Vec<bool>], x: isize, y: isize, default: bool) -> usize {
    let res = vec![
        value_at(image, x - 1, y - 1, default),
        value_at(image, x - 1, y, default),
        value_at(image, x - 1, y + 1, default),
        value_at(image, x, y - 1, default),
        value_at(image, x, y, default),
        value_at(image, x, y + 1, default),
        value_at(image, x + 1, y - 1, default),
        value_at(image, x + 1, y, default),
        value_at(image, x + 1, y + 1, default),
    ];

    // todo: simplify
    let base: usize = 2;

    res.iter()
        .enumerate()
        .filter(|(_, value)| **value)
        .map(|(index, _)| base.pow((9 - (index + 1)) as u32))
        .sum::<usize>()
}

fn value_at(image: &[Vec<bool>], x: isize, y: isize, default: bool) -> bool {
    let size = image.first().unwrap().len() as isize;
    if x < 0 || y < 0 || x >= size || y >= size {
        default
    } else {
        image[x as usize][y as usize]
    }
}

// todo: use chain here?
fn extend_image(image: Vec<Vec<bool>>, default: bool) -> (Vec<Vec<bool>>, usize) {
    let size = image.len() + 2;
    let padded_line = (0..size).map(|_| default).collect::<Vec<bool>>();

    let mut padded_image = vec![padded_line.clone()];

    let mut padded_input = image
        .into_iter()
        .map(|line| {
            let mut out = vec![default];
            out.extend(line.iter());
            out.push(default);
            out
        })
        .collect();

    padded_image.append(&mut padded_input);
    padded_image.push(padded_line);

    (padded_image, size)
}

#[cfg(test)]
mod tests {
    use crate::year2021::day20::get_pixel_value;
    use itertools::Itertools;

    #[test]
    fn test_get_n() {
        let image: Vec<Vec<bool>> = vec![
            vec![false, false, false],
            vec![true, false, false],
            vec![false, true, false],
        ];

        let decimal = get_pixel_value(&image, 1, 1, false);
        let expected = 34;

        assert_eq!(decimal, expected)
    }

    #[test]
    fn test_perm() {
        let items = 0..4;

        let mut perms = vec![];

        for perm in items.into_iter().permutations(2) {
            perms.push(perm);
        }

        let expected: Vec<Vec<i32>> = vec![vec![11]];

        assert_eq!(perms, expected)
    }
}
