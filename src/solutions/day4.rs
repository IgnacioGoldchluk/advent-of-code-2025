use std::{collections::HashSet, hash::Hash};

use crate::solutions::solution;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point(i64, i64);

pub struct Day4Solver;

impl solution::Solver for Day4Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: part2(input).to_string(),
        }
    }
}

fn part2(input: &str) -> usize {
    let mut points = get_rolls_points(input);
    let initial_size = points.len();

    loop {
        let to_remove = points
            .iter()
            .filter(|p| can_remove(p, &points))
            .cloned()
            .collect::<HashSet<Point>>();

        if to_remove.is_empty() {
            break;
        }

        for p in to_remove {
            points.remove(&p);
        }
    }

    initial_size - points.len()
}

fn part1(input: &str) -> usize {
    let points = get_rolls_points(input);
    points.iter().filter(|p| can_remove(p, &points)).count()
}

fn can_remove(point: &Point, points: &HashSet<Point>) -> bool {
    adjacent_rolls(point, points) < 4
}

fn adjacent_rolls(point: &Point, rolls_points: &HashSet<Point>) -> usize {
    adjacent_points(point)
        .iter()
        .filter(|p| rolls_points.contains(p))
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

fn get_rolls_points(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.char_indices()
                .map(move |(col, c)| (Point(row as i64, col as i64), c))
        })
        .filter_map(|(p, c)| if c == '@' { Some(p) } else { None })
        .collect::<HashSet<Point>>()
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
        assert_eq!(solution.part2, "43");
    }
}
