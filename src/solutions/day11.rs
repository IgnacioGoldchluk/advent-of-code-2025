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

use std::collections::{HashMap, HashSet};
type Nodes = HashMap<u32, HashSet<u32>>;

fn paths(nodes: &Nodes, in_node: u32, out_node: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    if in_node == out_node {
        return 1;
    }
    match cache.get(&in_node) {
        Some(val) => *val,
        None => {
            let total = nodes
                .get(&in_node)
                .unwrap_or(&HashSet::new())
                .iter()
                .fold(0, |acc, node| acc + paths(nodes, *node, out_node, cache));
            cache.insert(in_node, total);
            total
        }
    }
}

fn part2(input: &str) -> u64 {
    let nodes = parse(input);

    let svr = to_number("svr");
    let fft = to_number("fft");
    let dac = to_number("dac");
    let out = to_number("out");

    let p1 = paths(&nodes, svr, fft, &mut HashMap::new())
        * paths(&nodes, fft, dac, &mut HashMap::new())
        * paths(&nodes, dac, out, &mut HashMap::new());

    let p2 = paths(&nodes, svr, dac, &mut HashMap::new())
        * paths(&nodes, dac, fft, &mut HashMap::new())
        * paths(&nodes, fft, out, &mut HashMap::new());

    p1 + p2
}

fn part1(input: &str) -> u64 {
    let nodes = parse(input);
    let (you, out) = (to_number("you"), to_number("out"));
    paths(&nodes, you, out, &mut HashMap::new())
}

fn parse(input: &str) -> Nodes {
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
