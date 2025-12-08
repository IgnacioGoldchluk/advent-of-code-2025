use std::collections::{HashMap, HashSet, VecDeque};

use crate::solutions::solution;

pub struct Day8Solver;

impl solution::Solver for Day8Solver {
    fn solve(&self, input: &str) -> solution::Solution {
        solution::Solution {
            part1: part1(input).to_string(),
            part2: "".into(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point(i64, i64, i64);

type Distances = HashMap<(Point, Point), i64>;
type Graph = HashMap<Point, HashSet<Point>>;

fn part1(input: &str) -> u64 {
    let points = parse(input);
    let graph = build_graph(&sorted_distances(get_distances(&points)));

    let mut lengths: Vec<usize> = vec![];
    let mut seen: HashSet<Point> = HashSet::new();

    for point in graph.keys() {
        let mut total = 0;
        let mut to_visit = VecDeque::from([point]);

        while !to_visit.is_empty() {
            let new_point = to_visit.pop_front().unwrap();
            if seen.contains(new_point) {
                continue;
            }
            seen.insert(new_point.clone());
            total += 1;
            for p in graph.get(new_point).unwrap().iter() {
                to_visit.push_back(p);
            }
        }

        lengths.push(total);
    }

    lengths.sort();
    lengths.reverse();
    (lengths[0] * lengths[1] * lengths[2]) as u64
}

fn build_graph(connections: &[(Point, Point)]) -> Graph {
    let mut graph: Graph = HashMap::new();

    for (p1, p2) in connections.iter() {
        graph.entry(p1.clone()).or_default().insert(p2.clone());
        graph.entry(p2.clone()).or_default().insert(p1.clone());
    }

    graph
}

fn sorted_distances(distances: Distances) -> Vec<(Point, Point)> {
    let mut d: Vec<((Point, Point), i64)> = distances.into_iter().collect();
    d.sort_by(|(_points1, d1), (_points2, d2)| d1.cmp(d2));
    d.into_iter()
        .map(|(points, _distance)| points)
        .take(top_circuits())
        .collect()
}

fn get_distances(points: &[Point]) -> Distances {
    let mut distances: Distances = HashMap::new();

    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..].iter() {
            distances.insert((p1.clone(), p2.clone()), p1.distance(p2));
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
    }
}
