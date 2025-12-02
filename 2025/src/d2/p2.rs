use crate::{Input, Solution};
use std::collections::HashSet;

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
    ranges
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
                return nums.iter().filter(|n| is_in_ranges(**n, ranges)).sum();
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
