use crate::utils::union_find::UnionFind;
use std::{collections::BinaryHeap, str::FromStr};

#[derive(Debug)]
struct Coordinate {
    x: u64,
    y: u64,
    z: u64,
}

struct ParseCoordinateError;

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.trim().split(',').collect();
        if parts.len() != 3 {
            return Err(ParseCoordinateError);
        }
        let x = parts[0].parse::<u64>().map_err(|_| ParseCoordinateError)?;
        let y = parts[1].parse::<u64>().map_err(|_| ParseCoordinateError)?;
        let z = parts[2].parse::<u64>().map_err(|_| ParseCoordinateError)?;
        Ok(Coordinate { x, y, z })
    }
}

impl Coordinate {
    fn euclidean_distance(&self, other: &Coordinate) -> f64 {
        let dx = (self.x as i64 - other.x as i64) as f64;
        let dy = (self.y as i64 - other.y as i64) as f64;
        let dz = (self.z as i64 - other.z as i64) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct DistanceEntry {
    junction_ids: (usize, usize),
    distance: f64,
}

impl PartialEq for DistanceEntry {
    fn eq(&self, other: &Self) -> bool {
        self.junction_ids == other.junction_ids
    }
}

impl Eq for DistanceEntry {}

impl PartialOrd for DistanceEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistanceEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap()
            .then_with(|| self.junction_ids.cmp(&other.junction_ids))
    }
}

pub fn part1(input: &str) -> String {
    let junctions: Result<Vec<Coordinate>, ParseCoordinateError> = input
        .lines()
        .map(|line| line.parse::<Coordinate>())
        .collect();
    let junctions = match junctions {
        Ok(c) => c,
        Err(_) => return "Error parsing input".into(),
    };

    let num_connections = if junctions.len() > 100 { 1000 } else { 10 };

    let mut min_heap: BinaryHeap<std::cmp::Reverse<DistanceEntry>> = BinaryHeap::new();
    for (i, a) in junctions.iter().enumerate() {
        for (j, b) in junctions.iter().enumerate().skip(i + 1) {
            let dist = a.euclidean_distance(b);
            min_heap.push(std::cmp::Reverse(DistanceEntry {
                junction_ids: (i, j),
                distance: dist,
            }));
        }
    }

    let mut uf = UnionFind::new(junctions.len());
    let mut ckt_sizes = vec![1; junctions.len()];

    for _ in 0..num_connections {
        if let Some(std::cmp::Reverse(entry)) = min_heap.pop() {
            let (j0, j1) = entry.junction_ids;
            let p0 = uf.find(j0);
            let p1 = uf.find(j1);
            if p0 == p1 {
                continue;
            }
            let new_size = ckt_sizes[p0] + ckt_sizes[p1];
            ckt_sizes[p0] = 0;
            ckt_sizes[p1] = 0;
            uf.union(j0, j1);
            let new_parent = uf.find(j0);
            ckt_sizes[new_parent] = new_size;
        }
    }

    ckt_sizes.sort_unstable_by(|a, b| b.cmp(a));
    ckt_sizes.iter().take(3).product::<u64>().to_string()
}

pub fn part2(input: &str) -> String {
    let junctions: Result<Vec<Coordinate>, ParseCoordinateError> = input
        .lines()
        .map(|line| line.parse::<Coordinate>())
        .collect();
    let junctions = match junctions {
        Ok(c) => c,
        Err(_) => return "Error parsing input".into(),
    };

    let mut min_heap: BinaryHeap<std::cmp::Reverse<DistanceEntry>> = BinaryHeap::new();
    for (i, a) in junctions.iter().enumerate() {
        for (j, b) in junctions.iter().enumerate().skip(i + 1) {
            let dist = a.euclidean_distance(b);
            min_heap.push(std::cmp::Reverse(DistanceEntry {
                junction_ids: (i, j),
                distance: dist,
            }));
        }
    }

    let mut uf = UnionFind::new(junctions.len());

    let mut num_connections = 0;
    loop {
        if let Some(std::cmp::Reverse(entry)) = min_heap.pop() {
            let (j0, j1) = entry.junction_ids;
            let p0 = uf.find(j0);
            let p1 = uf.find(j1);
            if p0 == p1 {
                continue;
            }
            uf.union(j0, j1);
            num_connections += 1;
            if num_connections == junctions.len() - 1 {
                return (junctions[j0].x * junctions[j1].x).to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!(part1(input), "40");
    }

    #[test]
    fn test_part2() {
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
        assert_eq!(part2(input), "25272");
    }
}
