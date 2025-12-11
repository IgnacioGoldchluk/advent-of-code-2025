use std::collections::{HashMap, HashSet, VecDeque};

use crate::solutions::solution;

pub struct Day11Solver;

impl solution::Solver for Day11Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: part2(input).to_string(),
        }
    }
}

fn part2(input: &str) -> u64 {
    let nodes = parse(input);

    let in_node = to_number("svr");
    let out_node = to_number("out");
    let dac = to_number("dac");
    let fft = to_number("fft");

    let mut total = 0;
    let mut queue: VecDeque<(HashSet<u32>, u32)> = VecDeque::from([(HashSet::new(), in_node)]);

    while let Some((seen, node)) = queue.pop_front() {
        if seen.contains(&node) {
            continue;
        }
        if node == out_node {
            if seen.contains(&dac) && seen.contains(&fft) {
                total += 1;
            }
            continue;
        }
        for out in nodes.get(&node).unwrap_or(&HashSet::new()) {
            let mut seen_copy = seen.clone();
            seen_copy.insert(node);
            queue.push_back((seen_copy, *out));
        }
    }
    total
}

fn part1(input: &str) -> u64 {
    let nodes = parse(input);

    let in_node = to_number("you");
    let out_node = to_number("out");

    let mut total = 0;
    let mut queue: VecDeque<(HashSet<u32>, u32)> = VecDeque::from([(HashSet::new(), in_node)]);

    while let Some((seen, node)) = queue.pop_front() {
        if seen.contains(&node) {
            continue;
        }
        if node == out_node {
            total += 1;
            continue;
        }
        for out in nodes.get(&node).unwrap_or(&HashSet::new()) {
            let mut seen_copy = seen.clone();
            seen_copy.insert(node);
            queue.push_back((seen_copy, *out));
        }
    }
    total
}

fn parse(input: &str) -> HashMap<u32, HashSet<u32>> {
    input
        .lines()
        .map(|line| {
            let nodes: Vec<&str> = line.split(": ").collect();
            let in_node = to_number(nodes[0]);
            let out_nodes: HashSet<u32> = nodes[1].split(" ").map(to_number).collect();

            (in_node, out_nodes)
        })
        .collect()
}

fn to_number(node: &str) -> u32 {
    assert_eq!(node.len(), 3);
    node.char_indices()
        .fold(0, |acc, (idx, c)| acc + ((c as u32) << (idx * 8)))
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use crate::solutions::day11::Day11Solver;

    #[test]
    fn test_input_1() {
        let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

        let solution = Day11Solver.solve(input);
        assert_eq!(solution.part1, "5");
    }

    #[test]
    fn test_input_2() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;
        let solution = Day11Solver.solve(input);
        assert_eq!(solution.part2, "2");
    }
}
