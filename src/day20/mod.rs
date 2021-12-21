use itertools::Itertools;

// todo: change to chars?
fn parse_input(filename: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut input = vec![];
    let mut output = vec![];

    let mut has_split = false;

    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .for_each(|line| {
            if line == "" {
                has_split = true;
            } else {
                let mut values = line
                    .chars()
                    .map(|char| if char == '#' { 1 } else { 0 })
                    .collect::<Vec<usize>>();

                if has_split {
                    output.push(values)
                } else {
                    input.append(&mut values)
                }
            }
        });

    (input, output)
}

pub fn solve_1(filename: &str) -> String {
    let (codec, image) = parse_input(filename);

    let (first, new_default) = apply_codec(&codec, image, 0);
    let (second, _) = apply_codec(&codec, first, new_default);

    count_light_pixels(second).to_string()
}

pub fn solve_2(filename: &str) -> String {
    let (codec, image) = parse_input(filename);

    let mut image = image;
    let mut default = 0;

    for i in 0..50 {
        println!("Iteration: {}", i);
        let (updated_image, updated_default) = apply_codec(&codec, image, default);
        image = updated_image;
        default = updated_default;

    };

    count_light_pixels(image).to_string()
}

fn count_light_pixels(second: Vec<Vec<usize>>) -> usize {
    second
        .iter()
        .map(|row| (*row).iter().sum::<usize>())
        .sum::<usize>()
}

fn apply_codec(
    codec: &Vec<usize>,
    image: Vec<Vec<usize>>,
    default: usize,
) -> (Vec<Vec<usize>>, usize) {
    let (extended_image, extend_size) = extend_image(image, default);

    let output: Vec<Vec<usize>> = (0..extend_size)
        .into_iter()
        .map(|i| create_updated_row(codec, default, &extended_image, extend_size, i))
        .collect::<Vec<Vec<usize>>>();

    let new_default = if default==0 && *codec.first().unwrap() == 1 {
        1
    } else if default == 1 && *codec.last().unwrap() == 0 {
        0
    } else {
        default
    };

    (output, new_default)
}

fn create_updated_row(codec: &Vec<usize>, default: usize, extended_image: &Vec<Vec<usize>>, extend_size: usize, i: usize) -> Vec<usize> {
    (0..extend_size)
        .into_iter()
        .map(|j| get_pixel_value(&extended_image, i as isize, j as isize, default))
        .map(|value| codec[value])
        .collect()
}

fn get_pixel_value(image: &Vec<Vec<usize>>, x: isize, y: isize, default: usize) -> usize {

    let mut res = vec![];

    res.push(value_at(&image, x - 1, y - 1, default));
    res.push(value_at(&image, x - 1, y, default));
    res.push(value_at(&image, x - 1, y + 1, default));
    res.push(value_at(&image, x, y - 1, default));
    res.push(value_at(&image, x, y, default));
    res.push(value_at(&image, x, y + 1, default));
    res.push(value_at(&image, x + 1, y - 1, default));
    res.push(value_at(&image, x + 1, y, default));
    res.push(value_at(&image, x + 1, y + 1, default));

    let base: usize = 2;

    res
        .iter()
        .enumerate()
        .map(|(index, value)| value * base.pow((9 - (index + 1)) as u32))
        .sum::<usize>()

}

fn value_at(image: &Vec<Vec<usize>>, x: isize, y: isize, default: usize) -> usize {
    let size = image.first().unwrap().len() as isize;
    if x < 0 || y < 0 || x >= size || y >= size {
        default
    } else {
        image[x as usize][y as usize]
    }
}

// todo: use chain here?
fn extend_image(image: Vec<Vec<usize>>, default: usize) -> (Vec<Vec<usize>>, usize) {
    let size = image.len() + 2;
    let padded_line = (0..size).map(|_| default).collect::<Vec<usize>>();

    let mut padded_image = vec![];
    padded_image.push(padded_line.clone());

    let mut padded_input = image
        .into_iter()
        .map(|line| {
            let mut out = vec![default];
            out.extend(line.iter());
            out.push(default);
            out
        })
        .collect::<Vec<Vec<usize>>>();

    padded_image.append(&mut padded_input);
    padded_image.push(padded_line);

    (padded_image, size)
}

fn print_image(image: &Vec<Vec<usize>>) {
    println!();
    for row in image {
        println!("{:?}", row)
    }
    println!();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::day20::get_pixel_value;

    #[test]
    fn test_get_n() {
        let image: Vec<Vec<usize>> = vec![vec![0, 0, 0], vec![1, 0, 0], vec![0, 1, 0]];

        let decimal = get_pixel_value(&image, 1, 1, 0);
        let expected = 34;

        assert_eq!(decimal, expected)
    }

    #[test]
    fn test_perm() {
        //let items = vec![0, 1, 2, 3];
        let items = (0..4);

        let mut perms = vec![];

        for perm in items.into_iter().permutations(2) {
            perms.push(perm);
        }

        let expected: Vec<Vec<i32>> = vec![vec![11]];

        assert_eq!(perms, expected)

    }
}
