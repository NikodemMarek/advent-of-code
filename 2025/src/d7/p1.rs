use std::collections::HashSet;

use crate::{Input, Solution};

fn get_splitter_positions(row: &str) -> HashSet<usize> {
    row.chars()
        .enumerate()
        .filter(|(_, c)| *c == '^')
        .map(|(position, _)| position)
        .collect()
}

fn eval_splits(
    beam_positions: &HashSet<usize>,
    splitter_positions: &HashSet<usize>,
) -> (usize, HashSet<usize>) {
    beam_positions
        .iter()
        .fold((0, HashSet::new()), |(mut splits, mut new_beams), beam| {
            if splitter_positions.contains(beam) {
                splits += 1;
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
            } else {
                new_beams.insert(*beam);
            }

            (splits, new_beams)
        })
}

pub(crate) struct D7P1;
impl Solution<usize> for D7P1 {
    fn solution(input: impl Input) -> usize {
        let mut lines = input.lines();
        let source = lines.next().unwrap().find('S').unwrap();
        let splitters = lines.map(|l| get_splitter_positions(&l));

        splitters
            .fold(
                (0, HashSet::from([source])),
                |(splits, beam_positions), splitter_positions| {
                    let (new_splits, new_beams) = eval_splits(&beam_positions, &splitter_positions);
                    (splits + new_splits, new_beams)
                },
            )
            .0
    }
}

#[cfg(test)]
pub mod tests {
    use super::D7P1;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D7P1::solution(TestInput::new(vec![
                ".......S.......",
                "...............",
                ".......^.......",
                "...............",
                "......^.^......",
                "...............",
                ".....^.^.^.....",
                "...............",
                "....^.^...^....",
                "...............",
                "...^.^...^.^...",
                "...............",
                "..^...^.....^..",
                "...............",
                ".^.^.^.^.^...^.",
                "...............",
            ])),
            21
        );
    }
}