use std::collections::HashMap;

use crate::{Input, Solution};

fn parse_line(line: Box<str>) -> (Box<str>, Vec<Box<str>>) {
    let mut parts = line.split(' ');
    let root_name = parts.next().unwrap().to_string();
    let root_name = &root_name[..root_name.len() - 1];
    let children = parts.map(|v| v.to_owned().into_boxed_str()).collect();
    (root_name.to_owned().into_boxed_str(), children)
}

fn find_paths(check_node: &str, trees: &HashMap<Box<str>, Vec<Box<str>>>, depth: usize) -> usize {
    if check_node == "out" {
        return 1;
    }
    if depth == trees.len() {
        return 0;
    }

    trees
        .get(check_node)
        .unwrap()
        .iter()
        .map(|child| find_paths(child, trees, depth + 1))
        .sum()
}

pub(crate) struct D11P1;
impl Solution<usize> for D11P1 {
    fn solution(input: impl Input) -> usize {
        let trees = input
            .lines()
            .map(|l| parse_line(l))
            .collect::<HashMap<_, _>>();

        find_paths("you", &trees, 0)
    }
}
