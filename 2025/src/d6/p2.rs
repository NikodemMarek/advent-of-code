use crate::{Input, Solution};

fn get_column(lines: &[Box<str>]) -> impl Iterator<Item = Option<usize>> {
    (0..lines[0].len())
        .map(move |i| {
            String::into_boxed_str(
                lines
                    .iter()
                    .map(|l| l.as_bytes()[i] as char)
                    .collect::<String>()
                    .trim()
                    .to_string(),
            )
        })
        .map(|c| c.parse().ok())
}

pub(crate) struct D6P2;
impl Solution<usize> for D6P2 {
    fn solution(input: impl Input) -> usize {
        let mut lines = input.lines().collect::<Vec<_>>();
        let opers = lines.pop().unwrap();
        let mut operators = opers
            .split(' ')
            .map(|op| op.trim())
            .filter(|op| !op.is_empty())
            .map(|op| op == "+");
        let mut op = operators.next().unwrap();
        get_column(&lines)
            .chain(std::iter::once(None))
            .fold(
                (if op { 0 } else { 1 }, Vec::new()),
                move |(cr, mut results), num| {
                    let Some(num) = num else {
                        results.push(cr);
                        op = operators.next().unwrap_or(false);
                        return (if op { 0 } else { 1 }, results);
                    };

                    (if op { cr + num } else { cr * num }, results)
                },
            )
            .1
            .iter()
            .sum()
    }
}

#[cfg(test)]
pub mod tests {
    use super::D6P2;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D6P2::solution(TestInput::new(vec![
                "123 328  51 64 ",
                " 45 64  387 23 ",
                "  6 98  215 314",
                "*   +   *   +  ",
            ])),
            3263827
        );
    }
}
