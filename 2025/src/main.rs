mod d1;

use std::{
    env,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

trait Input {
    fn lines(&self) -> impl Iterator<Item = Box<str>>;
}

struct FileInput<'a>(&'a str);
impl<'a> Input for FileInput<'a> {
    fn lines(&self) -> impl Iterator<Item = Box<str>> {
        BufReader::new(File::open(self.0).expect("Could not open input file"))
            .lines()
            .map(|l| l.unwrap())
            .map(String::into_boxed_str)
    }
}

trait Solution<T: Display> {
    fn solution(input: impl Input) -> T;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <day> <part> <input_file>", args[0]);
        return;
    }

    let day = args[1]
        .parse::<u8>()
        .expect("Invalid day, please choose day in range 1-12");
    if day < 1 || day > 12 {
        panic!("Invalid day, please choose day in range 1-12");
    }

    let is_first_part = if args[2] == "1" {
        true
    } else if args[2] == "2" {
        false
    } else {
        panic!("Invalid part, choose either part 1 or 2")
    };

    let file_input = FileInput(&args[3]);
    let result = match (day, is_first_part) {
        (1, true) => d1::D1P1::solution(file_input),
        (1, false) => d1::D1P2::solution(file_input),
        _ => unreachable!(),
    };

    println!("Result -----------------------------------------------");
    println!("{result}");
    println!("------------------------------------------------------");
}

#[cfg(test)]
pub struct TestInput {
    lines: Vec<Box<str>>,
}

#[cfg(test)]
impl TestInput {
    pub fn new(lines: Vec<&str>) -> Self {
        TestInput {
            lines: lines
                .into_iter()
                .map(String::from)
                .map(String::into_boxed_str)
                .collect(),
        }
    }
}

#[cfg(test)]
impl Input for TestInput {
    fn lines(&self) -> impl Iterator<Item = Box<str>> {
        self.lines.clone().into_iter()
    }
}
