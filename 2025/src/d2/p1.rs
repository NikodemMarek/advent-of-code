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

#[cfg(test)]
pub mod tests {
    use super::D2P1;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D2P1::solution(TestInput::new(vec![
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
            1227775554
        );
    }
}