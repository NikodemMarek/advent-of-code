use std::collections::{HashMap, HashSet};

use crate::{Input, Solution};

fn get_splitter_positions(row: &str) -> HashSet<usize> {
    row.chars()
        .enumerate()
        .filter(|(_, c)| *c == '^')
        .map(|(position, _)| position)
        .collect()
}

fn eval_splits(
    beam_positions_and_paths: &HashMap<usize, usize>,
    splitter_positions: &HashSet<usize>,
) -> HashMap<usize, usize> {
    let mut beam_positions_and_paths_in_order = beam_positions_and_paths.iter().collect::<Vec<_>>();
    beam_positions_and_paths_in_order.sort();
    beam_positions_and_paths_in_order
        .iter()
        .fold(HashMap::new(), |mut new_beams, (beam, paths)| {
            if splitter_positions.contains(beam) {
                new_beams
                    .entry(*beam - 1)
                    .and_modify(|paths_to_splitter| *paths_to_splitter += *paths)
                    .or_insert(**paths);

                let leftover = beam_positions_and_paths.get(&(*beam + 1)).unwrap_or(&0);
                let sm = **paths + leftover;
                new_beams
                    .entry(*beam + 1)
                    .and_modify(|paths_to_splitter| *paths_to_splitter += sm)
                    .or_insert(sm);
            } else {
                new_beams.entry(**beam).or_insert(**paths);
            }

            new_beams
        })
}

pub(crate) struct D7P2;
impl Solution<usize> for D7P2 {
    fn solution(input: impl Input) -> usize {
        let mut lines = input.lines();
        let source = lines.next().unwrap().find('S').unwrap();
        let splitters = lines.map(|l| get_splitter_positions(&l));

        splitters
            .fold(
                HashMap::from([(source, 1)]),
                |beam_positions, splitter_positions| {
                    eval_splits(&beam_positions, &splitter_positions)
                },
            )
            .values()
            .sum()
    }
}

#[cfg(test)]
pub mod tests {
    use super::{D7P2, eval_splits, get_splitter_positions};
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D7P2::solution(TestInput::new(vec![
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
            40
        );
    }

    #[test]
    fn sums_layer() {
        assert_eq!(
            eval_splits(
                &std::collections::HashMap::from([
                    (2, 1),
                    (4, 5),
                    (6, 4),
                    (7, 3),
                    (8, 4),
                    (10, 2),
                    (12, 1),
                ]),
                &get_splitter_positions("..^...^.....^..")
            ),
            std::collections::HashMap::from([
                (1, 1),
                (3, 1),
                (4, 5),
                (5, 4),
                (7, 7),
                (8, 4),
                (10, 2),
                (11, 1),
                (13, 1),
            ]),
        );

        assert_eq!(
            eval_splits(
                &std::collections::HashMap::from([(3, 1), (5, 4), (7, 3), (8, 3), (9, 1), (11, 1)]),
                &get_splitter_positions("...^.^...^.^...")
            ),
            std::collections::HashMap::from([
                (2, 1),
                (4, 5),
                (6, 4),
                (7, 3),
                (8, 4),
                (10, 2),
                (12, 1)
            ]),
        );
    }
}