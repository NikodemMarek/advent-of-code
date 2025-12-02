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

pub(crate) struct D8P1;
impl Solution<usize> for D8P1 {
    fn solution(input: impl Input) -> usize {
        let how_many_connections = {
            #[cfg(not(test))]
            {
                1000
            }
            #[cfg(test)]
            {
                10
            }
        };

        let points = get_points(input.lines());

        let mut shortest_distances = BinaryHeap::with_capacity(1001);
        for i in 0..(points.len() - 1) {
            for j in (i + 1)..(points.len()) {
                let pa = &points[i];
                let pb = &points[j];
                let dist = pa.distance(pb);

                shortest_distances.push(Dist(dist, i, j));
                if shortest_distances.len() > 1000 {
                    shortest_distances.pop();
                }
            }
        }

        let circuits = shortest_distances
            .into_sorted_vec()
            .into_iter()
            .take(how_many_connections)
            .fold(Vec::new(), |mut circuits, Dist(_, a, b)| {
                merge_circuits(&mut circuits, (a, b));
                circuits
            });

        let mut circuit_lengths = circuits.into_iter().map(|c| c.len()).collect::<Vec<_>>();
        circuit_lengths.sort_by(|a, b| b.cmp(a));

        circuit_lengths
            .into_iter()
            .take(3)
            .reduce(|acc, cl| acc * cl)
            .unwrap()
    }
}



