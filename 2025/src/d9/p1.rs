use crate::{Input, Solution};

struct Coord(usize, usize);
impl From<Box<str>> for Coord {
    fn from(value: Box<str>) -> Self {
        let mut coords = value.split(',');
        Self(
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        )
    }
}

fn get_points(lines: impl Iterator<Item = Box<str>>) -> Vec<Coord> {
    lines.map(Coord::from).collect()
}

fn area(Coord(c1x, c1y): &Coord, Coord(c2x, c2y): &Coord) -> usize {
    let lx = (*c1x).checked_sub(*c2x).unwrap_or_else(|| *c2x - *c1x);
    let ly = (*c1y).checked_sub(*c2y).unwrap_or_else(|| *c2y - *c1y);
    (lx + 1) * (ly + 1)
}

fn walk_non_repeating(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..(n - 1)).flat_map(move |i| (i..n).map(move |j| (i, j)))
}

pub(crate) struct D9P1;
impl Solution<usize> for D9P1 {
    fn solution(input: impl Input) -> usize {
        let points = get_points(input.lines());

        walk_non_repeating(points.len())
            .map(|(i, j)| area(&points[i], &points[j]))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use super::D9P1;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D9P1::solution(TestInput::new(vec![
                "7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3",
            ])),
            50
        )
    }
}