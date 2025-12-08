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

fn is_in_ranges(num: usize, ranges: &[(usize, usize)]) -> bool {
    let mut start = 0;
    let mut end = ranges.len();

    while start < end {
        let mid = (start + end) / 2;
        let (ms, me) = ranges[mid];

        if me < num {
            start = mid + 1;
        } else if ms > num {
            end = mid;
        } else {
            return true;
        }
    }
    return false;
}

pub(crate) struct D5P1;
impl Solution<usize> for D5P1 {
    fn solution(input: impl Input) -> usize {
        let mut input = input.lines();

        let ranges = get_ranges(input.by_ref().take_while(|l| **l != *""));
        input
            .map(|ingredient| ingredient.parse().unwrap())
            .filter(|ingredient| is_in_ranges(*ingredient, &ranges))
            .count()
    }
}

#[cfg(test)]
pub mod tests {
    use super::D5P1;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D5P1::solution(TestInput::new(vec![
                "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
            ])),
            3
        );
    }
}
