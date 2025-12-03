use crate::solutions::solution;

pub struct Day3Solver;

impl solution::Solver for Day3Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input),
            part2: "0".into(),
        }
    }
}

fn part1(input: &str) -> String {
    input.lines().map(max_joltage).sum::<u32>().to_string()
}

fn max_joltage(bank: &str) -> u32 {
    let mut d = '0';
    let mut idx = 0;

    for (i, c) in bank[0..(bank.len() - 1)].chars().enumerate() {
        if c == '9' {
            d = c;
            idx = i;
            break;
        }
        if c > d {
            d = c;
            idx = i;
        }
    }
    let u = bank[idx + 1..].chars().max().unwrap();
    d.to_digit(10).unwrap() * 10 + u.to_digit(10).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day3::Day3Solver;

    use super::solution::Solver;

    #[test]
    fn test_solve() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        let solution = Day3Solver.solve(input);
        assert_eq!(solution.part1, "357");
        assert_eq!(solution.part2, "0");
    }
}
