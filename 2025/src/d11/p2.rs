use std::collections::HashMap;

use crate::{Input, Solution};

fn parse_line(line: Box<str>) -> (Box<str>, Vec<Box<str>>) {
    let mut parts = line.split(' ');
    let root_name = parts.next().unwrap().to_string();
    let root_name = &root_name[..root_name.len() - 1];
    let children = parts.map(|v| v.to_owned().into_boxed_str()).collect();
    (root_name.to_owned().into_boxed_str(), children)
}

struct FromNodeToOutThrough {
    none_before: HashMap<Box<str>, usize>,
    dac_before: HashMap<Box<str>, usize>,
    fft_before: HashMap<Box<str>, usize>,
    both_before: HashMap<Box<str>, usize>,
}
impl FromNodeToOutThrough {
    fn new() -> Self {
        FromNodeToOutThrough {
            none_before: HashMap::new(),
            dac_before: HashMap::new(),
            fft_before: HashMap::new(),
            both_before: HashMap::new(),
        }
    }

    fn insert(&mut self, name: &str, reached: Reached, paths_reaching_out: usize) {
        match reached {
            Reached::None => self.none_before.insert(name.into(), paths_reaching_out),
            Reached::DAC => self.dac_before.insert(name.into(), paths_reaching_out),
            Reached::FFT => self.fft_before.insert(name.into(), paths_reaching_out),
            Reached::Both => self.both_before.insert(name.into(), paths_reaching_out),
        };
    }

    fn get(&self, reached: Reached, name: &str) -> Option<usize> {
        match reached {
            Reached::None => self.none_before.get(name.into()),
            Reached::DAC => self.dac_before.get(name.into()),
            Reached::FFT => self.fft_before.get(name.into()),
            Reached::Both => self.both_before.get(name.into()),
        }
        .map(|r| *r)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Reached {
    None,
    DAC,
    FFT,
    Both,
}

fn find_paths(
    reachable_nodes: &mut FromNodeToOutThrough,
    check_node: &str,
    trees: &HashMap<Box<str>, Vec<Box<str>>>,
    reached: Reached,
    depth: usize,
) -> usize {
    if check_node == "out" {
        return if reached == Reached::Both { 1 } else { 0 };
    }
    if depth == trees.len() {
        return 0;
    }
    if let Some(cached) = reachable_nodes.get(reached, check_node) {
        return cached;
    };
    let reached = match (reached, check_node) {
        (Reached::None, "dac") => Reached::DAC,
        (Reached::None, "fft") => Reached::FFT,
        (Reached::FFT, "dac") | (Reached::DAC, "fft") => Reached::Both,
        (_, "dac") | (_, "fft") => return 0,
        _ => reached,
    };

    let paths_reaching_out = trees
        .get(check_node)
        .unwrap()
        .iter()
        .map(|child| find_paths(reachable_nodes, child, trees, reached, depth + 1))
        .sum();

    reachable_nodes.insert(check_node, reached, paths_reaching_out);
    paths_reaching_out
}

pub(crate) struct D11P2;
impl Solution<usize> for D11P2 {
    fn solution(input: impl Input) -> usize {
        let trees = input
            .lines()
            .map(|l| parse_line(l))
            .collect::<HashMap<_, _>>();

        let mut nodes_ref = FromNodeToOutThrough::new();
        find_paths(&mut nodes_ref, "svr", &trees, Reached::None, 0)
    }
}
