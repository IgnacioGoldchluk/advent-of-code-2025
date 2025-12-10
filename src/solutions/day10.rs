use std::collections::{HashSet, VecDeque};

use crate::solutions::solution;

pub struct Day10Solver;

impl solution::Solver for Day10Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: part2(input).to_string(),
        }
    }
}

fn part2(input: &str) -> u64 {
    let machines: Vec<Machine> = input.lines().map(Machine::from).collect();

    machines.iter().map(fewest_joltages).sum()
}

fn part1(input: &str) -> u64 {
    let machines: Vec<Machine> = input.lines().map(Machine::from).collect();

    machines.iter().map(fewest_presses).sum()
}

fn fewest_joltages(machine: &Machine) -> u64 {
    let joltages = &machine.joltages;
    let btn_positions: Vec<Vec<u64>> = machine.buttons.iter().map(|b| positions(*b)).collect();

    123
}

fn positions(button: u64) -> Vec<u64> {
    let mut b = button.clone();
    let mut res = vec![];
    let mut pos = 0;

    while b != 0 {
        if (b & 1) != 0 {
            res.push(pos);
        }
        b >>= 1;
        pos += 1;
    }

    res
}

fn fewest_presses(machine: &Machine) -> u64 {
    let seen: HashSet<u64> = HashSet::new();

    let mut queue: VecDeque<(u64, u64)> = VecDeque::new();

    machine.buttons.iter().for_each(|btn| {
        queue.push_back((1, *btn));
    });

    while !queue.is_empty() {
        let (count, val) = queue.pop_front().unwrap();

        if val == machine.target {
            return count;
        }
        machine.buttons.iter().for_each(|btn| {
            let new = val ^ btn;
            if !seen.contains(&new) {
                queue.push_back((count + 1, new));
            }
        });
    }
    panic!();
}

struct Machine {
    target: u64,
    buttons: Vec<u64>,
    joltages: Vec<u64>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let info: Vec<&str> = value.split(" ").collect();

        Self {
            target: parse_target(info[0]),
            joltages: parse_joltages(info[info.len() - 1]),
            buttons: parse_buttons(&info[1..info.len() - 1]),
        }
    }
}

fn parse_target(target: &str) -> u64 {
    let chars = target.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
    chars.chars().rev().fold(0, |acc, c| match c {
        '#' => (acc << 1) | 1,
        '.' => acc << 1,
        _ => panic!(),
    })
}

fn parse_joltages(jolt: &str) -> Vec<u64> {
    let n = jolt.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
    n.split(",").map(|num| num.parse().unwrap()).collect()
}

fn parse_buttons(buttons: &[&str]) -> Vec<u64> {
    buttons.iter().map(|b| parse_button(b)).collect()
}

fn parse_button(btn: &str) -> u64 {
    let n = btn.strip_prefix("(").unwrap().strip_suffix(")").unwrap();

    n.split(",")
        .map(|num| num.parse::<u64>().unwrap())
        .fold(0, |acc, num| acc | (1 << num))
}

#[cfg(test)]
mod tests {

    use super::solution::Solver;
    use crate::solutions::day10::Day10Solver;
    #[test]
    fn test_input() {
        let input = r###"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"###;

        let solution = Day10Solver.solve(input);
        assert_eq!(solution.part1, "7");
        assert_eq!(solution.part2, "")
    }
}
