use crate::{Input, Solution};

fn max_line_joltage(line: &str) -> u32 {
    let mut numbers_rev = line.chars().rev().map(|c| c.to_digit(10).unwrap());
    let starts = numbers_rev.next().unwrap();
    let startb = numbers_rev.next().unwrap();

    let (bigger, smaller) = numbers_rev.fold((startb, starts), |(bigger, smaller), current| {
        if current >= bigger {
            if smaller > bigger {
                (current, smaller)
            } else {
                (current, bigger)
            }
        } else {
            (bigger, smaller)
        }
    });

    bigger * 10 + smaller
}

pub(crate) struct D3P1;
impl Solution<u32> for D3P1 {
    fn solution(input: impl Input) -> u32 {
        input.lines().map(|l| max_line_joltage(&l)).sum()
    }
}
