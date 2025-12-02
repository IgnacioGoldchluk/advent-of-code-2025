use crate::solutions::solution;

enum Rotation {
    Left(i32),
    Right(i32),
}

struct State {
    position: i32,
    count: i32,
}

pub struct Day1Solver;

fn part1(rotations: &[Rotation]) -> State {
    let start = State {
        position: 50,
        count: 0,
    };

    rotations.iter().fold(start, |acc, cmd| {
        let new_pos = match cmd {
            Rotation::Left(val) => (acc.position - val).rem_euclid(100),
            Rotation::Right(val) => (acc.position + val).rem_euclid(100),
        };

        let new_count = match new_pos {
            0 => acc.count + 1,
            _ => acc.count,
        };

        State {
            position: new_pos,
            count: new_count,
        }
    })
}

fn part2(rotations: &[Rotation]) -> State {
    let start = State {
        position: 50,
        count: 0,
    };

    rotations.iter().fold(start, |acc, cmd| {
        let (full_rotations, rotation) = match cmd {
            Rotation::Left(val) => (val / 100, val.rem_euclid(100)),
            Rotation::Right(val) => (val / 100, val.rem_euclid(100)),
        };

        let (new_pos, rot) = match cmd {
            Rotation::Right(_) => {
                let rot = if rotation + acc.position >= 100 { 1 } else { 0 };
                ((acc.position + rotation).rem_euclid(100), rot)
            }
            Rotation::Left(_) => {
                let rot = if acc.position != 0 && (acc.position - rotation <= 0) {
                    1
                } else {
                    0
                };
                ((acc.position - rotation).rem_euclid(100), rot)
            }
        };

        State {
            position: new_pos,
            count: acc.count + full_rotations + rot,
        }
    })
}

impl solution::Solver for Day1Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        let parsed_input: Vec<Rotation> = input.lines().map(rotation).collect();

        let part1_solution = part1(&parsed_input);
        let part2_solution = part2(&parsed_input);

        solution::Solution {
            part1: part1_solution.count.to_string(),
            part2: part2_solution.count.to_string(),
        }
    }
}

fn rotation(value: &str) -> Rotation {
    let rotation = value.chars().next();
    let value: i32 = value[1..].parse().unwrap();

    match rotation {
        Some('L') => Rotation::Left(value),
        Some('R') => Rotation::Right(value),
        _ => panic!("Unexpected rotation"),
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        let solution = Day1Solver.solve(input);
        assert_eq!(solution.part1, "3");
        assert_eq!(solution.part2, "6");
    }
}
