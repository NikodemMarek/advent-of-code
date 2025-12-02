use std::collections::HashSet;

use crate::{Input, Solution};

#[derive(Debug)]
struct Present {
    elements: usize,
}
impl Present {
    fn parse(rows: impl Iterator<Item = Box<str>>) -> Self {
        let elements = rows.fold(0, |elements, row| {
            elements + row.chars().filter(|c| *c == '#').count()
        });
        Self { elements }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Region {
    size: (usize, usize),
    presents: Box<[usize]>,
}
impl Region {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(' ');
        let size = {
            let size = parts.next().unwrap();
            (size[..2].parse().unwrap(), size[3..5].parse().unwrap())
        };
        let presents = parts.map(|p| p.parse().unwrap()).collect();

        Self { size, presents }
    }

    fn area(&self) -> usize {
        self.size.0 * self.size.1
    }
}

fn parse_input(mut input: impl Iterator<Item = Box<str>>) -> (Box<[Present]>, HashSet<Region>) {
    let mut presents_iter = input.by_ref().take(6 * 5);
    let mut presents = Vec::new();
    while presents_iter.by_ref().next().is_some() {
        presents.push(Present::parse(presents_iter.by_ref().take(3)));
        presents_iter.next();
    }

    let regions = input.map(|l| Region::parse(&l)).collect();
    (presents.into(), regions)
}

pub(crate) struct D12P1;
impl Solution<usize> for D12P1 {
    fn solution(input: impl Input) -> usize {
        let (presents, regions) = parse_input(input.lines());

        regions
            .iter()
            .map(|region| {
                let presents_area = std::iter::zip(&region.presents, &presents)
                    .map(|(number_of_presents, present)| number_of_presents * present.elements)
                    .sum();
                (region, presents_area)
            })
            .filter(|(region, presents_space)| region.area() >= *presents_space)
            .count()
    }
}
