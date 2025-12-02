use std::collections::HashSet;

use crate::{Input, Solution};

struct Machine {
    lights_number: u8,
    lights: usize,
    buttons: HashSet<usize>,
}
impl From<Box<str>> for Machine {
    fn from(value: Box<str>) -> Self {
        let mut value = value.split(' ');
        let (lights_number, lights) = {
            let lights = value.next().unwrap();
            lights[1..lights.len() - 1].chars().map(|c| c == '.').fold(
                (0, 0),
                |(lights_number, lights), is_off| {
                    (
                        lights_number + 1,
                        lights | (if is_off { 0 } else { 1 } << lights_number),
                    )
                },
            )
        };
        let buttons = value
            .take_while(|v| v.starts_with('('))
            .map(|v| {
                let lights = &v[1..v.len() - 1];
                lights
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .fold(0, |mask, light: usize| mask | (1 << light))
            })
            .collect::<HashSet<_>>();
        Self {
            lights_number,
            lights,
            buttons,
        }
    }
}

fn get_lowest_clicks(
    Machine {
        lights,
        buttons,
        lights_number,
    }: Machine,
) -> usize {
    if lights == 0 {
        return 0;
    }

    let mut combinations = HashSet::from([0]);
    let mut clicks = 1;

    loop {
        let mut new_combinations = HashSet::with_capacity(1 << (lights_number + 1));
        let did_find = combinations
            .iter()
            .flat_map(|combination| buttons.iter().map(move |button| combination ^ button))
            .fold(false, |is_found, new_combination| {
                if is_found || new_combination == lights {
                    return true;
                }
                new_combinations.insert(new_combination);
                false
            });
        if did_find {
            return clicks;
        }
        combinations = new_combinations;
        clicks += 1;
    }
}

pub(crate) struct D10P1;
impl Solution<usize> for D10P1 {
    fn solution(input: impl Input) -> usize {
        input
            .lines()
            .map(|l| Machine::from(l))
            .map(|machine| get_lowest_clicks(machine))
            .sum()
    }
}
