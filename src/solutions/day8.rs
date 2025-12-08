use std::collections::HashMap;

use crate::solutions::solution;

pub struct Day8Solver;

impl solution::Solver for Day8Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: part2(input).to_string(),
        }
    }
}

struct Point(i64, i64, i64);

type Distances = HashMap<(usize, usize), i64>;

fn part2(input: &str) -> u64 {
    let points = parse(input);
    let target = points.len() - 1;

    let distances = sorted_distances(get_distances(&points));
    let mut connections = UnionFind::new(points.len());

    let mut connection_count = 0;
    for (i, j) in distances.iter() {
        if connections.find(*i) != connections.find(*j) {
            connection_count += 1;
            if connection_count == target {
                return (points[*i].0 * points[*j].0) as u64;
            }
            connections.connect(*i, *j);
        }
    }
    panic!("Unreachable");
}

fn part1(input: &str) -> u64 {
    let points = parse(input);
    let distances = sorted_distances(get_distances(&points));
    let mut connections = UnionFind::new(points.len());

    for (i, j) in distances.iter().take(top_circuits()) {
        connections.connect(*i, *j);
    }

    let mut freqs: HashMap<usize, u64> = HashMap::new();
    for idx in 0..points.len() {
        *freqs.entry(connections.find(idx)).or_default() += 1;
    }

    let mut v: Vec<u64> = freqs.values().copied().collect();
    v.sort();
    v.reverse();
    v.iter().take(3).product()
}

fn sorted_distances(distances: Distances) -> Vec<(usize, usize)> {
    let mut d: Vec<((usize, usize), i64)> = distances.into_iter().collect();
    d.sort_by(|(_points1, d1), (_points2, d2)| d1.cmp(d2));
    d.into_iter().map(|(points, _distance)| points).collect()
}

fn get_distances(points: &[Point]) -> Distances {
    let mut distances: Distances = HashMap::new();

    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points[i + 1..].iter().enumerate() {
            distances.insert((i, j + i + 1), p1.distance(p2));
        }
    }
    distances
}

fn parse(input: &str) -> Vec<Point> {
    input.lines().map(Point::from).collect()
}

impl Point {
    fn distance(&self, other: &Self) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let nums: Vec<i64> = value.split(",").map(|n| n.parse().unwrap()).collect();
        match nums.as_slice() {
            [x, y, z] => Self(*x, *y, *z),
            _ => panic!("Expected length 3 array"),
        }
    }
}

struct UnionFind {
    indexes: Vec<usize>,
}

impl UnionFind {
    fn new(capacity: usize) -> Self {
        Self {
            indexes: (0..capacity + 1).collect(),
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        if idx == self.indexes[idx] {
            return idx;
        }
        self.indexes[idx] = self.find(self.indexes[idx]);
        self.indexes[idx]
    }

    fn connect(&mut self, i: usize, j: usize) {
        let value = self.find(j);
        let idx = self.find(i);
        self.indexes[idx] = value;
    }
}

fn top_circuits() -> usize {
    if cfg!(test) { 10 } else { 1000 }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use crate::solutions::day8::Day8Solver;

    #[test]
    fn test_input() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        let solution = Day8Solver.solve(input);
        assert_eq!(solution.part1, "40");
        assert_eq!(solution.part2, "25272");
    }
}
