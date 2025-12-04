use std::collections::HashMap;

use crate::solutions::solution;

#[derive(PartialEq, Eq, Hash)]
struct Point(i64, i64);

enum PointValue {
    Empty,
    PaperRoll,
}

type Grid = HashMap<Point, PointValue>;

pub struct Day4Solver;

impl solution::Solver for Day4Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: "".into(),
        }
    }
}

fn part1(input: &str) -> usize {
    let rolls_grid = grid(input);

    rolls_grid
        .keys()
        .filter(|point| is_roll(point, &rolls_grid) && adjacent_rolls(point, &rolls_grid) < 4)
        .count()
}

fn is_roll(point: &Point, grid: &Grid) -> bool {
    matches!(grid.get(point), Some(PointValue::PaperRoll))
}

fn adjacent_rolls(point: &Point, grid: &Grid) -> usize {
    adjacent_points(point)
        .iter()
        .filter(|p| is_roll(p, grid))
        .count()
}

fn adjacent_points(point: &Point) -> Vec<Point> {
    let mut result = Vec::new();

    for x in point.0 - 1..=point.0 + 1 {
        for y in point.1 - 1..=point.1 + 1 {
            if x == point.0 && y == point.1 {
                continue;
            }
            result.push(Point(x, y));
        }
    }
    result
}

fn point_value(c: char) -> PointValue {
    match c {
        '.' => PointValue::Empty,
        '@' => PointValue::PaperRoll,
        _ => panic!("Unexpected character: {c}"),
    }
}

fn grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, c)| (Point(row as i64, col as i64), point_value(c)))
        })
        .collect::<Grid>()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day4::Day4Solver;

    use super::solution::Solver;

    #[test]
    fn test_solve() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        let solution = Day4Solver.solve(input);
        assert_eq!(solution.part1, "13");
    }
}
