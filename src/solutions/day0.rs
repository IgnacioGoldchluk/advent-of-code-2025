use crate::solutions::solution;

pub struct Day0Solver;
impl solution::Solver for Day0Solver {
    fn solve(self, input: String) -> solution::Solution {
        let values: Vec<&str> = input.lines().collect();
        solution::Solution {
            part1: values[0].to_string(),
            part2: values[1].to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let solution = Day0Solver.solve("1\n2".to_string());
        assert_eq!(solution.part1, "1");
        assert_eq!(solution.part2, "2");
    }
}
