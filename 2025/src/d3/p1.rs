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

#[cfg(test)]
pub mod tests {
    use super::{D3P1, max_line_joltage};
    use crate::{Solution, TestInput};

    #[test]
    fn should_calculate_max_line_joltage() {
        assert_eq!(max_line_joltage("987654321111111"), 98);
        assert_eq!(max_line_joltage("811111111111119"), 89);
        assert_eq!(max_line_joltage("234234234234278"), 78);
        assert_eq!(max_line_joltage("818181911112111"), 92);
    }

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D3P1::solution(TestInput::new(vec![
                "987654321111111",
                "811111111111119",
                "234234234234278",
                "818181911112111",
            ])),
            357
        )
    }
}

