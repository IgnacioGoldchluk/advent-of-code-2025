use crate::solutions::solution;

pub struct Day6Solver;

impl solution::Solver for Day6Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: part2(input).to_string(),
        }
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

fn part1(input: &str) -> u64 {
    let (numbers, operators) = parse(input);

    (0..numbers[0].len())
        .map(|col| {
            let operation = get_func(&operators[col]);
            numbers.iter().map(|n| n[col]).reduce(operation).unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = {
        let lines: Vec<&str> = input.lines().collect();
        lines[..lines.len() - 1]
            .iter()
            .map(|s| s.chars().collect())
            .collect()
    };

    let mut operands = input
        .lines()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(to_operation);

    let mut total = 0;
    let mut nums: Vec<u64> = Vec::new();

    for col in 0..lines[0].len() {
        let n: String = lines.iter().map(|l| l[col]).collect();

        if n.trim() == "" {
            let op = operands.next().unwrap();
            total += nums.clone().into_iter().reduce(get_func(&op)).unwrap();
            nums.clear();
        } else {
            nums.push(n.trim().parse().unwrap());
        }
    }

    let op = operands.next().unwrap();
    total + nums.clone().into_iter().reduce(get_func(&op)).unwrap()
}

fn get_func(operator: &Op) -> impl Fn(u64, u64) -> u64 {
    match operator {
        Op::Add => |x, y| x + y,
        Op::Mul => |x, y| x * y,
    }
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let mut reversed = input.lines().rev();

    let operations: Vec<Op> = reversed
        .next()
        .unwrap()
        .split_whitespace()
        .map(to_operation)
        .collect();

    let numbers: Vec<Vec<u64>> = reversed
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    (numbers, operations)
}

fn to_operation(input: &str) -> Op {
    match input {
        "+" => Op::Add,
        "*" => Op::Mul,
        _ => panic!("Unknown operation: {input}"),
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use crate::solutions::day6::Day6Solver;

    #[test]
    fn test_input() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

        let solution = Day6Solver.solve(input);
        assert_eq!(solution.part1, "4277556");
    }
}
