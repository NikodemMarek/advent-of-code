use std::collections::{HashMap, HashSet};

use crate::{Input, Solution};

struct Machine {
    buttons: Box<[Button]>,
    joltages: Joltages,
}
impl From<Box<str>> for Machine {
    fn from(value: Box<str>) -> Self {
        let mut value = value.split(' ').peekable();
        value.next().unwrap();

        // Collect button indices (not masks yet)
        let mut button_indices = Vec::new();
        while value.peek().unwrap().starts_with('(') {
            let button = value.next().unwrap();
            let button = &button[1..button.len() - 1];
            let indices: Vec<usize> = button.split(',').map(|n| n.parse().unwrap()).collect();
            button_indices.push(indices);
        }

        // Parse joltages to get the length
        let joltages = {
            let joltages = value.next().unwrap();
            let joltages = &joltages[1..joltages.len() - 1];
            joltages
                .split(',')
                .map(|j| j.parse().unwrap())
                .collect::<Vec<_>>()
        };
        let num_joltages = joltages.len();

        // Create button masks with correct offset
        let buttons: Vec<Button> = button_indices
            .into_iter()
            .map(|indices| {
                let mask = indices
                    .into_iter()
                    .fold(0, |mask, i| mask | (1 << (num_joltages - 1 - i)));
                Button::new(mask)
            })
            .collect();

        Self {
            buttons: buttons.into(),
            joltages: Joltages::new(&joltages),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Button {
    mask: u16,
}
impl Button {
    fn new(mask: u16) -> Self {
        Self { mask }
    }

    fn get_value(&self) -> [usize; 16] {
        std::array::from_fn(|i| ((self.mask >> (15 - i)) & 1) as usize)
    }
}
impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B({:0>10b})", self.mask)
    }
}

struct ButtonsCombination(HashSet<Button>);
impl ButtonsCombination {
    fn joltages(&self) -> Joltages {
        Joltages(
            self.0
                .iter()
                .map(|button| button.get_value())
                .fold([0; 16], |r, button| {
                    std::iter::zip(r, button)
                        .map(|(rv, bv)| rv + bv)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                }),
        )
    }
}

struct Patterns(HashMap<u16, Vec<ButtonsCombination>>);
fn genereate_patterns(buttons: &[Button]) -> Patterns {
    let mut patterns = Patterns(HashMap::new());
    fn masks_recursive(
        patterns: &mut Patterns,
        buttons: &[Button],
        si: usize,
        current: &mut HashSet<Button>,
        current_mask: u16,
    ) {
        patterns
            .0
            .entry(current_mask)
            .and_modify(|previous| {
                previous.push(ButtonsCombination(current.clone()));
            })
            .or_insert(Vec::from([ButtonsCombination(current.clone())]));

        for i in si..buttons.len() {
            let button = buttons[i];
            current.insert(button);

            masks_recursive(
                patterns,
                buttons,
                i + 1,
                current,
                current_mask ^ button.mask,
            );

            current.remove(&button);
        }
    }
    masks_recursive(&mut patterns, buttons, 0, &mut HashSet::new(), 0);
    patterns
}

#[derive(Eq, Hash, PartialEq)]
struct Joltages([usize; 16]);
impl Joltages {
    fn new(joltages: &[usize]) -> Joltages {
        let mut res = [0; 16];
        res[16 - joltages.len()..].copy_from_slice(joltages);
        Joltages(res)
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|j| *j == 0)
    }

    fn odd_mask(&self) -> u16 {
        self.0
            .iter()
            .map(|j| if j % 2 == 0 { 0 } else { 1 })
            .fold(0, |mask, jm| (mask << 1) | jm)
    }

    fn sub(&self, value: Joltages) -> Option<Joltages> {
        let mut result = [0; 16];
        for i in 0..16 {
            result[i] = self.0[i].checked_sub(value.0[i])?;
        }
        Some(Joltages(result))
    }

    fn halved(&self) -> Joltages {
        Joltages(self.0.map(|v| v / 2))
    }
}

fn configure(joltages: Joltages, patterns: &Patterns) -> Option<usize> {
    fn get_min_presses(
        cache: &mut HashMap<Joltages, Option<usize>>,
        patterns: &Patterns,
        joltages: Joltages,
    ) -> Option<usize> {
        if joltages.is_zero() {
            return Some(0);
        }

        if let Some(cached) = cache.get(&joltages) {
            return *cached;
        };

        let odd_mask = joltages.odd_mask();
        let mut minimum_presses = None;
        for buttons_combination in patterns.0.get(&odd_mask)? {
            let buttons_joltages = buttons_combination.joltages();
            let Some(reminder) = joltages.sub(buttons_joltages) else {
                continue;
            };

            let halved = reminder.halved();
            let Some(min_presses_to_halved) = get_min_presses(cache, patterns, halved) else {
                continue;
            };

            let presses = min_presses_to_halved * 2 + buttons_combination.0.len();
            minimum_presses = Some(minimum_presses.map_or(presses, |m| std::cmp::min(m, presses)));
        }

        cache.insert(joltages, minimum_presses);
        minimum_presses
    }

    let mut cache = HashMap::new();
    get_min_presses(&mut cache, patterns, joltages)
}

pub(crate) struct D10P2;
impl Solution<usize> for D10P2 {
    fn solution(input: impl Input) -> usize {
        input
            .lines()
            .map(|l| Machine::from(l))
            .map(|Machine { buttons, joltages }| {
                configure(joltages, &genereate_patterns(&buttons)).unwrap()
            })
            .sum()
    }
}
