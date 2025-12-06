use crate::solutions::solution;

pub struct Day6Solver;

impl solution::Solver for Day6Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: "".into(),
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

    let add = |x, y| x + y;
    let mul = |x, y| x * y;

    (0..numbers[0].len())
        .map(|col| {
            let operation = match operators[col] {
                Op::Add => add,
                Op::Mul => mul,
            };

            numbers.iter().map(|n| n[col]).reduce(operation).unwrap()
        })
        .sum()
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
