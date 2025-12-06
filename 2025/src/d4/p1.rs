use crate::{Input, Solution};

fn carth(m: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..m).flat_map(move |m| (0..n).map(move |n| (m, n)))
}

fn neighbours_array(array: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let size_m = array.len();
    let size_n = array[0].len();
    let mut neighbours = (0..size_m)
        .map(|_| (0..size_n).map(|_| 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut update_at = |m: i64, n: i64| {
        if m >= 0 && m < size_m as i64 && n >= 0 && n < size_n as i64 {
            neighbours[m as usize][n as usize] += 1;
        }
    };

    for (m, n) in carth(size_m, size_n) {
        if array[m][n] == '@' {
            let m = m as i64;
            let n = n as i64;
            update_at(m - 1, n - 1);
            update_at(m - 1, n);
            update_at(m - 1, n + 1);

            update_at(m, n - 1);
            update_at(m, n + 1);

            update_at(m + 1, n - 1);
            update_at(m + 1, n);
            update_at(m + 1, n + 1);
        }
    }

    neighbours
}

pub(crate) struct D4P1;
impl Solution<usize> for D4P1 {
    fn solution(input: impl Input) -> usize {
        let array = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let neighbours = neighbours_array(&array);

        carth(array.len(), array[0].len())
            .filter(|(m, n)| neighbours[*m][*n] < 4 && array[*m][*n] == '@')
            .count()
    }
}

#[cfg(test)]
pub mod tests {
    use super::D4P1;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D4P1::solution(TestInput::new(vec![
                "..@@.@@@@.",
                "@@@.@.@.@@",
                "@@@@@.@.@@",
                "@.@@@@..@.",
                "@@.@@@@.@@",
                ".@@@@@@@.@",
                ".@.@.@.@@@",
                "@.@@@.@@@@",
                ".@@@@@@@@.",
                "@.@.@@@.@.",
            ])),
            13
        );
    }
}
