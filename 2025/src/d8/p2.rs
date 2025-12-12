use std::collections::{BinaryHeap, HashSet};

use crate::{Input, Solution};

struct Coord(usize, usize, usize);
impl From<Box<str>> for Coord {
    fn from(value: Box<str>) -> Self {
        let mut coords = value.split(',');
        Self(
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        )
    }
}

fn get_points(lines: impl Iterator<Item = Box<str>>) -> Vec<Coord> {
    lines.map(Coord::from).collect()
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Dist(f64, usize, usize);
impl Eq for Dist {}
impl Ord for Dist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::total_cmp(&self.0, &other.0)
    }
}

impl Coord {
    fn distance(&self, other: &Self) -> f64 {
        let x = self
            .0
            .checked_sub(other.0)
            .unwrap_or_else(|| other.0 - self.0);
        let y = self
            .1
            .checked_sub(other.1)
            .unwrap_or_else(|| other.1 - self.1);
        let z = self
            .2
            .checked_sub(other.2)
            .unwrap_or_else(|| other.2 - self.2);
        ((x * x + y * y + z * z) as f64).sqrt()
    }
}

fn merge_circuits(circuits: &mut Vec<HashSet<usize>>, (a, b): (usize, usize)) {
    let circuit_with_a = circuits.iter().position(|circuit| circuit.contains(&a));
    let circuit_with_b = circuits.iter().position(|circuit| circuit.contains(&b));

    match (circuit_with_a, circuit_with_b) {
        (Some(a_circuit), Some(b_circuit)) if a_circuit == b_circuit => {}
        (Some(a_circuit), Some(b_circuit)) => {
            let b_circuit_content = circuits.get(b_circuit).unwrap().clone();
            circuits
                .get_mut(a_circuit)
                .unwrap()
                .extend(b_circuit_content);
            circuits.remove(b_circuit);
        }
        (Some(a_circuit), None) => {
            circuits.get_mut(a_circuit).unwrap().insert(b);
        }
        (None, Some(b_circuit)) => {
            circuits.get_mut(b_circuit).unwrap().insert(a);
        }
        (None, None) => {
            circuits.push(HashSet::from([a, b]));
        }
    }
}

pub(crate) struct D8P2;
impl Solution<usize> for D8P2 {
    fn solution(input: impl Input) -> usize {
        let points = get_points(input.lines());

        let mut shortest_distances = BinaryHeap::with_capacity(1000 * 1000);
        for i in 0..(points.len() - 1) {
            for j in (i + 1)..(points.len()) {
                let pa = &points[i];
                let pb = &points[j];
                let dist = pa.distance(pb);

                shortest_distances.push(Dist(dist, i, j));
            }
        }

        let mut last_two = (0, 0);
        let circuits = shortest_distances.into_sorted_vec().into_iter().fold(
            Vec::<HashSet<_>>::new(),
            |mut circuits, Dist(_, a, b)| {
                if circuits.len() == 1 && circuits[0].len() == points.len() {
                    return circuits;
                }
                merge_circuits(&mut circuits, (a, b));
                last_two = (a, b);
                circuits
            },
        );

        let point_a = &points[last_two.0];
        let point_b = &points[last_two.1];
        point_a.0 * point_b.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::D8P2;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D8P2::solution(TestInput::new(vec![
                "162,817,812",
                "57,618,57",
                "906,360,560",
                "592,479,940",
                "352,342,300",
                "466,668,158",
                "542,29,236",
                "431,825,988",
                "739,650,466",
                "52,470,668",
                "216,146,977",
                "819,987,18",
                "117,168,530",
                "805,96,715",
                "346,949,466",
                "970,615,88",
                "941,993,340",
                "862,61,35",
                "984,92,344",
                "425,690,689",
            ])),
            25272
        );
    }
}