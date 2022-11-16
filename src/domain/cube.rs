use std::collections::HashSet;
use std::fmt;
use crate::domain::line::Line;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Cube(pub Line, pub Line, pub Line);

impl Cube {

    pub fn new_optional(x: &Option<Line>, y: &Option<Line>, z: &Option<Line>) -> Option<Cube> {

        if let Some(x) = x {
            if let Some(y) = y {
                if let Some(z) = z {
                    return Some(Cube(*x, *y, *z))
                }
            }
        }

        None
    }

    pub fn new_symmetric(start: isize, end: isize) -> Cube {
        let line = Line::new(start, end);
        Cube(line.unwrap(), line.unwrap(), line.unwrap())
    }

    pub fn overlaps(&self, other: &Cube) -> bool {
        self.0.overlaps(&other.0)
            && self.1.overlaps(&other.1)
            && self.2.overlaps(&other.2)
    }

    /*
    pub fn intersection(&self, other: &Cube) -> Option<Cube> {

        let x_line = self.0.intersection(&other.0);
        let y_line = self.1.intersection(&other.1);
        let z_line = self.2.intersection(&other.2);

        Cube::new_optional(&x_line, &y_line, &z_line)
    }

     */

    pub fn subtract(&self, other: &Cube) -> Vec<Cube> {

        let mut cubes = HashSet::new();

        let x_diff = self.0.diff(&other.0);
        let y_diff = self.1.diff(&other.1);
        let z_diff = self.2.diff(&other.2);

        let x_intersect = self.0.intersection(&other.0);
        let y_intersect = self.1.intersection(&other.1);
        let z_intersect = self.2.intersection(&other.2);


        for x_line in &x_diff {

            let x_line = &Some(*x_line);

            // internals
            cubes.insert(Cube::new_optional(x_line, &y_intersect, &z_intersect));

            // external y's
            for y_line in &y_diff {
                let y_line = &Some(*y_line);
                cubes.insert(Cube::new_optional(x_line, y_line, &z_intersect));

                // external yz's
                for z_line in &z_diff {
                    let z_line = &Some(*z_line);
                    cubes.insert(Cube::new_optional(x_line, y_line, z_line));
                }
            }

            // external z's
            for z_line in &z_diff {
                let z_line = &Some(*z_line);
                cubes.insert(Cube::new_optional(x_line, &y_intersect, z_line));
            }
        }

        for y_line in &y_diff {
            // internals
            let y_line = &Some(*y_line);
            cubes.insert(Cube::new_optional(&x_intersect, y_line, &z_intersect));

            // external z's
            for z_line in &z_diff {
                let z_line = &Some(*z_line);
                cubes.insert(Cube::new_optional(&x_intersect, y_line, z_line));
            }
        }

        for z_line in &y_diff {
            // internals
            let z_line = &Some(*z_line);
            cubes.insert(Cube::new_optional(&x_intersect, &y_intersect, z_line));
        }

        cubes.into_iter().flatten().collect()
    }


    // todo: replace lines with vectors?
    // todo: make more fine-grained by removing duplicates?

    pub fn diff(&self, other: &Cube) -> Vec<Cube> {
        let x_diff = self.0.diff(&other.0);
        let y_diff = self.1.diff(&other.1);
        let z_diff = self.2.diff(&other.2);

        let mut cubes = x_diff
            .into_iter()
            .map(|x_line| Cube(x_line, self.1, self.2))
            .collect::<Vec<Cube>>();

        let y_iterator = y_diff
            .into_iter()
            .map(|y_line| Cube(self.0, y_line, self.2));

        let z_iterator = z_diff
            .into_iter()
            .map(|z_line| Cube(self.0, self.1, z_line));

        cubes.extend(y_iterator);
        cubes.extend(z_iterator);
        cubes
    }

    pub fn size(&self) -> usize {
        self.0.length() * self.1.length() * self.2.length()
    }

    pub fn get_cuboids(&self) -> Vec<(isize, isize, isize)> {

        let mut res = vec![];

        for x in self.0.as_range() {
            for y in self.1.as_range() {
                for z in self.2.as_range() {
                    res.push((x, y, z));
                }
            }
        }

        res
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={}, y={}, z={}", self.0, self.1, self.2)
    }
}


#[cfg(test)]
mod test_cube {
    use crate::domain::cube::Cube;
    use crate::domain::line::Line;

    #[test]
    fn test_cube_overlaps_other() {
        let line = Line::new(0, 10);
        let cube = Cube(line.unwrap(), line.unwrap(), line.unwrap());

        let other_line = Line::new(-5, 5);
        let other = Cube(other_line.unwrap(), other_line.unwrap(), other_line.unwrap());

        assert!(cube.overlaps(&other))
    }

    #[test]
    fn test_cube_does_not_overlap_other() {
        let line = Line::new(0, 10);
        let cube = Cube(line.unwrap(), line.unwrap(), line.unwrap());

        let other_line = Line::new(-5, -1);
        let other = Cube(other_line.unwrap(), other_line.unwrap(), other_line.unwrap());

        assert!(!cube.overlaps(&other))
    }

    #[test]
    fn test_cube_does_not_overlap_other2() {
        let line = Line::new(-50, 50);
        let cube = Cube(line.unwrap(), line.unwrap(), line.unwrap());

        let x = Line::new(-57795, -6158).unwrap();
        let y = Line::new(29564, 72030).unwrap();
        let z = Line::new(20435, 90618).unwrap();

        let other = Cube(x, y, z);

        assert!(!other.overlaps(&cube));
        assert!(!cube.overlaps(&other))
    }

    #[test]
    fn test_cube_does_overlap_other3() {
        let line = Line::new(-50, 50);
        let cube = Cube(line.unwrap(), line.unwrap(), line.unwrap());

        let x = Line::new(-14, 36).unwrap();
        let y = Line::new(-6, 44).unwrap();
        let z = Line::new(-16, 29).unwrap();

        let other = Cube(x, y, z);

        println!("Cube: \n{}", cube);
        println!("Other: \n{}", other);

        assert!(cube.overlaps(&other));
        assert!(other.overlaps(&cube));
    }

    #[test]
    fn test_cube_diff() {

        let cube = Cube::new_symmetric(0, 10);
        let other = Cube::new_symmetric(4, 6);

        let diff = cube.diff(&other);
        let expected: Vec<Cube> = vec![];

        for diff_cube in &diff {
            println!("{:?}", diff_cube)
        }

        assert_eq!(diff, expected)
    }

    #[test]
    fn test_cube_x_diff() {

        let cube = Cube::new_symmetric(0, 10);
        let other = Cube::new_symmetric(4, 6);

        let diff = cube.subtract(&other);
        let _expected: Vec<Cube> = vec![];

        for diff_cube in &diff {
            println!("{}", diff_cube)
        }

        println!("\nNumber of cubes: {}", diff.len());

        assert_eq!(diff.len(), 38)
    }

    #[test]
    fn test_cube_x_diff_2() {

        let cube = Cube::new_symmetric( 10, 11);
        let other = Cube::new_symmetric(10, 10);

        let diff = cube.subtract(&other);
        let _expected: Vec<Cube> = vec![];

        for diff_cube in &diff {
            //println!("{}", diff_cube);
            for (cx, cy, cz) in diff_cube.get_cuboids() {
                println!("{},{},{}", cx, cy, cz);
            }
        }

        println!("\nNumber of cubes: {}", &diff.len());

        assert_eq!(diff.len(), 7)
    }

    #[test]
    fn test_subtract_unit_cube() {

        let cube = Cube::new_symmetric( 0, 1);
        let unit = Cube::new_symmetric(0, 0);

        let subtracted = cube.subtract(&unit);

        assert_eq!(subtracted.len(), 7);
        assert!(!subtracted.contains(&unit));
    }

    #[test]
    fn test_subtract_super_cube() {

        let cube = Cube::new_symmetric( 0, 1);
        let super_cube = Cube::new_symmetric(0, 2);

        let subtracted = cube.subtract(&super_cube);

        assert!(subtracted.is_empty());
    }


    #[test]
    fn test_subtract_sub_cube() {

        let cube = Cube::new_symmetric( 0, 10);
        let sub_cube = Cube::new_symmetric(4, 6);

        let subtracted = cube.subtract(&sub_cube);

        assert_eq!(subtracted, vec![sub_cube]);
    }
}
