use crate::solutions::solution;

pub struct Day9Solver;

impl solution::Solver for Day9Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: "".into(),
        }
    }
}

fn part1(input: &str) -> u64 {
    let coords: Vec<Coord> = input.lines().map(Coord::from).collect();

    let mut max_val = u64::MIN;

    for i in 0..(coords.len() - 1) {
        for j in i + 1..coords.len() {
            max_val = max_val.max(coords[i].area(&coords[j]));
        }
    }
    max_val
}

struct Coord(u64, u64);

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let nums: Vec<u64> = value.split(",").map(|n| n.parse().unwrap()).collect();
        match nums.as_slice() {
            [x, y] => Self(*x, *y),
            _ => panic!("Expected 2-element vector"),
        }
    }
}

impl Coord {
    fn area(&self, other: &Self) -> u64 {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use crate::solutions::day9::Day9Solver;

    #[test]
    fn test_input() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
        let solution = Day9Solver.solve(input);
        assert_eq!(solution.part1, "50");
    }
}
