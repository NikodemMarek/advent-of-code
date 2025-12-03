use crate::{Input, Solution};

const DIAL_START: u8 = 50;
const DIAL_MAX: u8 = 100;

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
impl From<Rotation> for u8 {
    fn from(rotation: Rotation) -> Self {
        let max: u16 = DIAL_MAX as u16;
        (if rotation.0 {
            rotation.1
        } else {
            max - (rotation.1 % max)
        } % max) as u8
    }
}

pub(crate) struct D1P1;
impl Solution<usize> for D1P1 {
    fn solution(input: impl Input) -> usize {
        input
            .lines()
            .map(|l| (&*l).into())
            .map(|r: Rotation| r.into())
            .fold((DIAL_START, 0), |(position, zeros), r: u8| {
                let position = (position + r) % DIAL_MAX;
                (position, if position == 0 { zeros + 1 } else { zeros })
            })
            .1
    }
}

#[cfg(test)]
pub mod tests {
    use super::{D1P1, Rotation};
    use crate::{Solution, TestInput};

    #[test]
    fn rotation_parses_correctly() {
        assert_eq!(Into::<Rotation>::into("R16"), Rotation(true, 16));
        assert_eq!(Into::<Rotation>::into("L16"), Rotation(false, 16));
        assert_eq!(Into::<Rotation>::into("L0"), Rotation(false, 0));
        assert_eq!(Into::<Rotation>::into("R256"), Rotation(true, 256));
    }

    #[test]
    fn rotation_converts_to_simple_moves() {
        assert_eq!(Into::<u8>::into(Rotation(true, 16)), 16);
        assert_eq!(Into::<u8>::into(Rotation(false, 16)), 84);
        assert_eq!(Into::<u8>::into(Rotation(true, 99)), 99);
        assert_eq!(Into::<u8>::into(Rotation(false, 99)), 1);
        assert_eq!(Into::<u8>::into(Rotation(true, 0)), 0);
        assert_eq!(Into::<u8>::into(Rotation(false, 0)), 0);
        assert_eq!(Into::<u8>::into(Rotation(true, 100)), 0);
        assert_eq!(Into::<u8>::into(Rotation(false, 100)), 0);
        assert_eq!(Into::<u8>::into(Rotation(true, 116)), 16);
        assert_eq!(Into::<u8>::into(Rotation(false, 116)), 84);
    }

    #[test]
    fn solution_evaluates_correctly() {
        assert_eq!(D1P1::solution(TestInput::new(vec!["R50"])), 1);
        assert_eq!(D1P1::solution(TestInput::new(vec!["L50"])), 1);
        assert_eq!(D1P1::solution(TestInput::new(vec!["R50", "R16", "L16"])), 2);
        assert_eq!(D1P1::solution(TestInput::new(vec!["R550"])), 1);
        assert_eq!(
            D1P1::solution(TestInput::new(vec!["R50", "L100", "R100"])),
            3
        );
        assert_eq!(D1P1::solution(TestInput::new(vec!["R25", "R15", "R10"])), 1);
        assert_eq!(D1P1::solution(TestInput::new(vec!["L25", "L15", "L10"])), 1);
        assert_eq!(D1P1::solution(TestInput::new(vec!["L25", "L15", "L10"])), 1);
        assert_eq!(
            D1P1::solution(TestInput::new(vec![
                "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
            ])),
            3
        );
    }
}
