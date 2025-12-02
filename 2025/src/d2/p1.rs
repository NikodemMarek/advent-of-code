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

fn double(digits: u32, num: usize) -> usize {
    num + num * 10usize.pow(digits)
}

fn all_nums(digits: u32) -> impl Iterator<Item = usize> {
    let min = 10usize.pow(digits - 1);
    let max = min * 10;
    min..max
}

fn sum_doubles(ranges: &[(usize, usize)]) -> usize {
    let mut sum = 0;
    let mut digits = 1;
    let max = ranges.last().unwrap().1;
    loop {
        for num in all_nums(digits) {
            let doubled = double(digits, num);
            if doubled > max {
                return sum;
            }
            if is_in_ranges(doubled, ranges) {
                sum += doubled;
            }
        }
        digits += 1;
    }
}

fn is_in_ranges(num: usize, ranges: &[(usize, usize)]) -> bool {
    for (start, end) in ranges {
        if &num >= start && &num <= end {
            return true;
        }
    }
    return false;
}

pub(crate) struct D2P1;
impl Solution<usize> for D2P1 {
    fn solution(input: impl Input) -> usize {
        sum_doubles(&get_ranges(input))
    }
}



