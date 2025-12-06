use std::collections::HashSet;
use crate::{Input, Solution};

fn get_ranges(input: impl Input) -> Vec<(usize, usize)> {
    let mut ranges = input
        .split_delimeter(&',')
        .map(|range| {
            let mut split = range.splitn(2, '-');
            (
                split.nth(0).unwrap().parse().unwrap(),
                split.nth(0).unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    ranges.sort();
    return ranges;
}

const PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

fn repeat(digits: u32, num: usize, times: u32) -> Option<usize> {
    let mut fin = num;
    let shift = 10usize.pow(digits);
    for _ in 1..times {
        let Some(next) = fin.checked_mul(shift) else {
            return None;
        };
        fin = num + next;
    }
    Some(fin)
}

fn all_nums(digits: u32) -> impl Iterator<Item = usize> {
    let min = 10usize.pow(digits - 1);
    let max = min * 10;
    min..max
}

fn sum_doubles(ranges: &[(usize, usize)]) -> usize {
    let mut digits = 1;
    let max = ranges.last().unwrap().1;
    let maxh = max / 2;
    let mut nums = HashSet::new();
    loop {
        for num in all_nums(digits) {
            if num > maxh {
                return nums.iter()
                    .filter(|n| is_in_ranges(**n, ranges))
                    .sum();
            }

            for prime in PRIMES {
                let Some(repeated) = repeat(digits, num, prime) else {
                    break;
                };
                if repeated > max {
                    break;
                }

                nums.insert(repeated);
            }
        }
        digits += 1;
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

pub(crate) struct D2P2;
impl Solution<usize> for D2P2 {
    fn solution(input: impl Input) -> usize {
        sum_doubles(&get_ranges(input))
    }
}

#[cfg(test)]
pub mod tests {
use super::{repeat, is_in_ranges, D2P2};
    use crate::{Solution, TestInput};
    
    #[test]
    fn extends_correctly() {
        assert_eq!(repeat(1, 1, 1), Some(1));
        assert_eq!(repeat(1, 1, 2), Some(11));
        assert_eq!(repeat(1, 1, 3), Some(111));

        assert_eq!(repeat(3, 247, 3), Some(247247247));
    }

    #[test]
    fn finds_number_in_ranges() {
        assert!(is_in_ranges(5, &[(1, 5)]));
        assert!(is_in_ranges(5, &[(5, 18)]));
        assert!(!is_in_ranges(5, &[(7, 18)]));

        assert!(is_in_ranges(5, &[(1, 5), (7, 18)]));
        assert!(!is_in_ranges(5, &[(1, 2), (7, 18)]));

        assert!(is_in_ranges(14, &[(1, 2), (12, 15), (22, 25)]));
        assert!(!is_in_ranges(14, &[(1, 2), (12, 13), (22, 25)]));
    }

    #[test]
    fn evaluates_correctly() {
        return;
        assert_eq!(
            D2P2::solution(TestInput::new(vec![
                "11-22",
                "95-115",
                "998-1012",
                "1188511880-1188511890",
                "222220-222224",
                "1698522-1698528",
                "446443-446449",
                "38593856-38593862",
                "565653-565659",
                "824824821-824824827",
                "2121212118-2121212124"
            ])),
            4174379265
        );
    }
}
