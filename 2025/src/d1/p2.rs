use crate::{Input, Solution};

const DIAL_START: i16 = 50;
const DIAL_MAX: i16 = 100;

#[derive(PartialEq, Debug)]
struct Rotation(bool, u16);
impl Into<Rotation> for &str {
    fn into(self) -> Rotation {
        Rotation(
            self.as_bytes()[0] as char == 'R',
            self[1..].parse().unwrap(),
        )
    }
}
impl From<Rotation> for i16 {
    fn from(Rotation(is_right, by): Rotation) -> Self {
        if is_right { by as i16 } else { -(by as i16) }
    }
}

pub(crate) struct D1P2;
impl Solution<usize> for D1P2 {
    fn solution(input: impl Input) -> usize {
        input
            .lines()
            .map(|l| (&*l).into())
            .map(|r: Rotation| r.into())
            .fold((DIAL_START, 0), |(position, zeros), r: i16| {
                let mut full_turns = (r / DIAL_MAX as i16).abs();
                let reminder = r % DIAL_MAX as i16;

                let new_position = (position
                    + if reminder > 0 {
                        reminder
                    } else {
                        DIAL_MAX + reminder
                    })
                    % DIAL_MAX;
                if reminder > 0 {
                    // 3 -> 6 = false
                    if position != 0 && new_position < position || new_position == 0 {
                        full_turns += 1;
                    }
                    // 3 -> 1 = true
                    // 0 -> 5 = false
                    // 5 -> 0 = true
                } else if reminder < 0 {
                    // 1 <- 3 = false
                    // 6 <- 3 = true
                    if position != 0 && new_position > position || new_position == 0 {
                        full_turns += 1;
                    }
                    // 5 <- 0 = false
                    // 0 <- 5 = true
                }

                (new_position, zeros + full_turns as usize)
            })
            .1
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::TestInput;

    #[test]
    fn solution_evaluates_correctly() {
        assert_eq!(D1P2::solution(TestInput::new(vec!["R0"])), 0);
        assert_eq!(D1P2::solution(TestInput::new(vec!["L0"])), 0);
        assert_eq!(D1P2::solution(TestInput::new(vec!["R50", "R0"])), 1);
        assert_eq!(D1P2::solution(TestInput::new(vec!["L50", "L0"])), 1);
        assert_eq!(D1P2::solution(TestInput::new(vec!["R50"])), 1);
        assert_eq!(D1P2::solution(TestInput::new(vec!["L50"])), 1);
        assert_eq!(D1P2::solution(TestInput::new(vec!["R50", "R16", "L16"])), 2);
        assert_eq!(D1P2::solution(TestInput::new(vec!["R550"])), 6);
        assert_eq!(
            D1P2::solution(TestInput::new(vec!["R50", "L100", "R100"])),
            3
        );
        assert_eq!(D1P2::solution(TestInput::new(vec!["R25", "R15", "R10"])), 1);
        assert_eq!(D1P2::solution(TestInput::new(vec!["L25", "L15", "L10"])), 1);
        assert_eq!(D1P2::solution(TestInput::new(vec!["L25", "L15", "L10"])), 1);
        assert_eq!(
            D1P2::solution(TestInput::new(vec![
                "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
            ])),
            6
        );
    }
}
