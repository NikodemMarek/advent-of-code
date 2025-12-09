use crate::{Input, Solution};

fn get_column(line: &str) -> Vec<Box<str>> {
    line.split(' ')
        .map(|s| s.trim().into())
        .filter(|s: &Box<str>| !s.is_empty())
        .collect()
}

pub(crate) struct D6P1;
impl Solution<usize> for D6P1 {
    fn solution(input: impl Input) -> usize {
        let mut columns = input.lines().map(|l| get_column(&l)).collect::<Vec<_>>();
        let operators = columns.pop().unwrap();

        let mut values = columns
            .into_iter()
            .map(|c| c.into_iter().map(|v| v.parse::<usize>().unwrap()));

        let initial_value = values.next().unwrap().collect::<Vec<_>>();
        values
            .fold(initial_value, move |prev, values| {
                std::iter::zip(operators.clone(), std::iter::zip(prev, values))
                    .map(|(operator, (prev, value))| {
                        if operator == "*".into() {
                            prev * value
                        } else {
                            prev + value
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .iter()
            .sum()
    }
}

#[cfg(test)]
pub mod tests {
    use super::D6P1;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D6P1::solution(TestInput::new(vec![
                "123 328  51 64 ",
                " 45 64  387 23 ",
                "  6 98  215 314",
                "*   +   *   +  ",
            ])),
            4277556
        );
    }
}
