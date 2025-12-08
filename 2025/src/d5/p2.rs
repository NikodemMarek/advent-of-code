use crate::{Input, Solution};

fn get_ranges<'a>(lines: impl Iterator<Item = Box<str>>) -> Vec<(usize, usize)> {
    let mut ranges = lines
        .map(|range| {
            let mut split = range.splitn(2, '-');
            (
                split.nth(0).unwrap().parse().unwrap(),
                split.nth(0).unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    ranges.sort();
    ranges.into_iter().fold(Vec::new(), |mut ranges, curr| {
        if ranges.is_empty() {
            ranges.push(curr);
        }

        let prev = ranges.pop().unwrap();
        ranges.extend(merge_range(prev, curr));
        ranges
    })
}

fn merge_range(
    (a_start, a_end): (usize, usize),
    (b_start, b_end): (usize, usize),
) -> Vec<(usize, usize)> {
    if a_end < b_start {
        Vec::from([(a_start, a_end), (b_start, b_end)])
    } else {
        Vec::from([(std::cmp::min(a_start, b_start), std::cmp::max(a_end, b_end))])
    }
}

pub(crate) struct D5P2;
impl Solution<usize> for D5P2 {
    fn solution(input: impl Input) -> usize {
        get_ranges(input.lines().by_ref().take_while(|l| **l != *""))
            .iter()
            .fold(0, |sum, (start, end)| sum + end - start + 1)
    }
}

#[cfg(test)]
pub mod tests {
    use super::D5P2;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D5P2::solution(TestInput::new(vec![
                "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
            ])),
            14
        );
    }
}
