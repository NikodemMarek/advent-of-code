mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

use std::{
    env,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

trait Input {
    fn lines(&self) -> impl Iterator<Item = Box<str>>;
    fn split_delimeter(&self, delimeter: &'static char) -> impl Iterator<Item = Box<str>> {
        self.lines()
            .map(|l| {
                l.split(*delimeter)
                    .map(|e| String::into_boxed_str(e.to_owned()))
                    .collect::<Vec<_>>()
            })
            .flatten()
    }
    fn chars(&self) -> impl Iterator<Item = char> {
        self.lines()
            .map(|l| l.chars().collect::<Vec<_>>().into_iter())
            .flatten()
    }
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
        (1, true) => format!("{}", d1::D1P1::solution(file_input)),
        (1, false) => format!("{}", d1::D1P2::solution(file_input)),
        (2, true) => format!("{}", d2::D2P1::solution(file_input)),
        (2, false) => format!("{}", d2::D2P2::solution(file_input)),
        (3, true) => format!("{}", d3::D3P1::solution(file_input)),
        (3, false) => format!("{}", d3::D3P2::solution(file_input)),
        (4, true) => format!("{}", d4::D4P1::solution(file_input)),
        (4, false) => format!("{}", d4::D4P2::solution(file_input)),
        (5, true) => format!("{}", d5::D5P1::solution(file_input)),
        (5, false) => format!("{}", d5::D5P2::solution(file_input)),
        _ => unreachable!(),
    };

    println!("Result -----------------------------------------------");
    println!("{result}");
    println!("------------------------------------------------------");
}

#[cfg(test)]
pub struct TestInput {
    elements: Vec<Box<str>>,
}

#[cfg(test)]
impl TestInput {
    pub fn new(elements: Vec<&str>) -> Self {
        TestInput {
            elements: elements
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
        self.elements.clone().into_iter()
    }
}
