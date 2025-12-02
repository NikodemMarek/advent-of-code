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
