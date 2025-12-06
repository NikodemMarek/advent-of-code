use crate::{Input, Solution};

fn carth(m: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..m).flat_map(move |m| (0..n).map(move |n| (m, n)))
}

fn neighbours_array(array: &Vec<Vec<char>>) -> Vec<Vec<u8>> {
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

fn remove_rolls(array: &mut Vec<Vec<char>>, neighbours: &mut Vec<Vec<u8>>) -> usize {
    let size_m = array.len();
    let size_n = array[0].len();

    carth(array.len(), array[0].len())
        .filter(|(m, n)| {
            if neighbours[*m][*n] < 4 && array[*m][*n] == '@' {
                let mut update_at = |m: i64, n: i64| {
                    if m >= 0 && m < size_m as i64 && n >= 0 && n < size_n as i64 {
                        neighbours[m as usize][n as usize] =
                            neighbours[m as usize][n as usize].saturating_sub(1);
                    }
                };
                array[*m][*n] = '.';

                let m = *m as i64;
                let n = *n as i64;
                update_at(m - 1, n - 1);
                update_at(m - 1, n);
                update_at(m - 1, n + 1);

                update_at(m, n - 1);
                update_at(m, n + 1);

                update_at(m + 1, n - 1);
                update_at(m + 1, n);
                update_at(m + 1, n + 1);

                true
            } else {
                false
            }
        })
        .count()
}

pub(crate) struct D4P2;
impl Solution<usize> for D4P2 {
    fn solution(input: impl Input) -> usize {
        let mut array = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut neighbours = neighbours_array(&array);

        let mut removed_total = 0;
        loop {
            let removed = remove_rolls(&mut array, &mut neighbours);
            if removed == 0 {
                return removed_total;
            }
            removed_total += removed;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::D4P2;
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D4P2::solution(TestInput::new(vec![
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
            43
        );
    }
}
