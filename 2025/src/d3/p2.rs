use crate::{Input, Solution};

fn insert_digit(mut digits: [usize; 12], digit: usize) -> [usize; 12] {
    let mut carry = Some(digit);
    let mut i = 0;

    while let Some(digit) = carry {
        if i >= 12 {
            break;
        }

        if digits[i] <= digit {
            carry = Some(digits[i]);
            digits[i] = digit;
        } else {
            carry = None;
        }

        i += 1;
    }

    digits
}

fn max_line_joltage(line: &str) -> usize {
    let mut numbers_rev = line.chars().rev().map(|c| c.to_digit(10).unwrap() as usize);

    let digits = numbers_rev.by_ref().take(12).collect::<Vec<_>>();
    let mut digits: [usize; 12] = digits.try_into().unwrap();
    digits.reverse();

    let digits = numbers_rev.fold(digits, |digits, current| {
        if current >= digits[0] {
            insert_digit(digits, current)
        } else {
            digits
        }
    });

    digits.iter().fold(0, |sum, digit| sum * 10 + digit)
}

pub(crate) struct D3P2;
impl Solution<usize> for D3P2 {
    fn solution(input: impl Input) -> usize {
        input.lines().map(|l| max_line_joltage(&l)).sum()
    }
}
