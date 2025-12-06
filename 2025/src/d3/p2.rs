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

#[cfg(test)]
pub mod tests {
    use super::{D3P2, insert_digit, max_line_joltage};
    use crate::{Solution, TestInput};

    #[test]
    fn should_insert_digit() {
        assert_eq!(
            insert_digit([1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6], 1),
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6]
        );
        assert_eq!(
            insert_digit([1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6], 8),
            [8, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6]
        );
        assert_eq!(
            insert_digit([5, 4, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6], 8),
            [8, 5, 4, 4, 5, 6, 7, 8, 9, 8, 7, 6]
        );
        assert_eq!(
            insert_digit([5, 4, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6], 8),
            [8, 5, 4, 4, 5, 6, 7, 8, 9, 8, 7, 6]
        );
        assert_eq!(
            insert_digit([1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 8),
            [8, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
        assert_eq!(
            insert_digit([8, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 1),
            [8, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
        assert_eq!(
            insert_digit([8, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 8),
            [8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
    }

    #[test]
    fn should_calculate_max_line_joltage() {
        assert_eq!(max_line_joltage("987654321111111"), 987654321111);
        assert_eq!(max_line_joltage("811111111111119"), 811111111119);
        assert_eq!(max_line_joltage("234234234234278"), 434234234278);
        assert_eq!(max_line_joltage("818181911112111"), 888911112111);
    }

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D3P2::solution(TestInput::new(vec![
                "987654321111111",
                "811111111111119",
                "234234234234278",
                "818181911112111",
            ])),
            3121910778619
        );
    }
}
