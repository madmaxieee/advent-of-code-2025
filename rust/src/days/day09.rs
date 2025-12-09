#[allow(dead_code, unused_variables)]
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: u64,
    y: u64,
}

impl Coordinate {
    fn neighbors(&self) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            neighbors.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
        }
        neighbors.push(Coordinate {
            x: self.x + 1,
            y: self.y,
        });
        if self.y > 0 {
            neighbors.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
        }
        neighbors.push(Coordinate {
            x: self.x,
            y: self.y + 1,
        });
        neighbors
    }
}

struct ParseCoordinateError;

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.trim().split(',').collect();
        if parts.len() != 2 {
            return Err(ParseCoordinateError);
        }
        let x = parts[0].parse::<u64>().map_err(|_| ParseCoordinateError)?;
        let y = parts[1].parse::<u64>().map_err(|_| ParseCoordinateError)?;
        Ok(Coordinate { x, y })
    }
}

impl Coordinate {
    fn rectangle_area(&self, other: &Coordinate) -> u64 {
        let dx = (self.x as i64 - other.x as i64).abs() + 1;
        let dy = (self.y as i64 - other.y as i64).abs() + 1;
        (dx.abs() * dy.abs()) as u64
    }
}

struct AreaEntry {
    tile_ids: (usize, usize),
    area: u64,
}

impl PartialEq for AreaEntry {
    fn eq(&self, other: &Self) -> bool {
        self.tile_ids == other.tile_ids
    }
}

impl Eq for AreaEntry {}

impl PartialOrd for AreaEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AreaEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

struct Bounds {
    x_min: u64,
    x_max: u64,
    y_min: u64,
    y_max: u64,
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            x_min: u64::MAX,
            x_max: u64::MIN,
            y_min: u64::MAX,
            y_max: u64::MIN,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Bounds {
    #[allow(dead_code)]
    fn closest_edge(&self, c: &Coordinate) -> Direction {
        let dist_left = c.x - self.x_min;
        let dist_right = self.x_max - c.x;
        let dist_top = c.y - self.y_min;
        let dist_bottom = self.y_max - c.y;

        let min_dist = dist_left.min(dist_right).min(dist_top).min(dist_bottom);

        if min_dist == dist_left {
            Direction::Left
        } else if min_dist == dist_right {
            Direction::Right
        } else if min_dist == dist_top {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}

struct Raycaster {
    tiles: Vec<Coordinate>,
    horizontal_line_segments: HashMap<usize, Vec<(Coordinate, Coordinate)>>,
    vertical_line_segments: HashMap<usize, Vec<(Coordinate, Coordinate)>>,
    bounds: Bounds,
}

impl Raycaster {
    fn new(tiles: Vec<Coordinate>) -> Self {
        let bounds = tiles.iter().fold(Bounds::default(), |mut bounds, tile| {
            if tile.x < bounds.x_min {
                bounds.x_min = tile.x;
            }
            if tile.x > bounds.x_max {
                bounds.x_max = tile.x;
            }
            if tile.y < bounds.y_min {
                bounds.y_min = tile.y;
            }
            if tile.y > bounds.y_max {
                bounds.y_max = tile.y;
            }
            bounds
        });

        let mut horizontal_line_segments: HashMap<usize, Vec<(Coordinate, Coordinate)>> =
            HashMap::new();
        let mut vertical_line_segments: HashMap<usize, Vec<(Coordinate, Coordinate)>> =
            HashMap::new();

        let wrapped_tiles = {
            let mut v = tiles.clone();
            v.push(tiles[0]);
            v
        };

        for tile in wrapped_tiles.windows(2) {
            let a = &tile[0];
            let b = &tile[1];
            if a.y == b.y {
                let (x_start, x_end) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
                if let Some(segments) = horizontal_line_segments.get_mut(&(a.y as usize)) {
                    segments.push((
                        Coordinate { x: x_start, y: a.y },
                        Coordinate { x: x_end, y: a.y },
                    ));
                } else {
                    horizontal_line_segments.insert(
                        a.y as usize,
                        vec![(
                            Coordinate { x: x_start, y: a.y },
                            Coordinate { x: x_end, y: a.y },
                        )],
                    );
                }
            } else if a.x == b.x {
                let (y_start, y_end) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
                if let Some(segments) = vertical_line_segments.get_mut(&(a.x as usize)) {
                    segments.push((
                        Coordinate { x: a.x, y: y_start },
                        Coordinate { x: a.x, y: y_end },
                    ));
                } else {
                    vertical_line_segments.insert(
                        a.x as usize,
                        vec![(
                            Coordinate { x: a.x, y: y_start },
                            Coordinate { x: a.x, y: y_end },
                        )],
                    );
                }
            }
        }

        Raycaster {
            tiles,
            horizontal_line_segments,
            vertical_line_segments,
            bounds,
        }
    }

    fn is_on_border(&self, coord: &Coordinate) -> bool {
        for seg in self
            .horizontal_line_segments
            .get(&(coord.y as usize))
            .unwrap_or(&Vec::new())
        {
            if seg.0.x <= coord.x && coord.x <= seg.1.x {
                return true;
            }
        }
        for seg in self
            .vertical_line_segments
            .get(&(coord.x as usize))
            .unwrap_or(&Vec::new())
        {
            if seg.0.y <= coord.y && coord.y <= seg.1.y {
                return true;
            }
        }
        false
    }

    fn is_enclosed(&self, coord: &Coordinate, cache: &mut HashMap<Coordinate, bool>) -> bool {
        if let Some(val) = cache.get(coord) {
            return *val;
        }

        if self.is_on_border(coord) {
            cache.insert(*coord, true);
            return true;
        }

        let mut result_from_neighbors: Option<bool> = None;
        for neighbor in coord.neighbors() {
            if !self.is_on_border(&neighbor)
                && let Some(val) = cache.get(&neighbor)
            {
                result_from_neighbors = Some(*val);
                break;
            }
        }
        if let Some(result) = result_from_neighbors {
            cache.insert(*coord, result);
            return result;
        }

        let direction = self.bounds.closest_edge(coord);

        let ray: Vec<Coordinate> = match direction {
            Direction::Left => (self.bounds.x_min..=coord.x)
                .map(|x| Coordinate { x, y: coord.y })
                .collect(),
            Direction::Right => (coord.x..=self.bounds.x_max)
                .map(|x| Coordinate { x, y: coord.y })
                .collect(),
            Direction::Up => (self.bounds.y_min..=coord.y)
                .map(|y| Coordinate { x: coord.x, y })
                .collect(),
            Direction::Down => (coord.y..=self.bounds.y_max)
                .map(|y| Coordinate { x: coord.x, y })
                .collect(),
        };

        let mut intersections = 0;

        let mut last_corner_direction: Option<Direction> = None;
        for c in ray {
            match direction {
                Direction::Left | Direction::Right => {
                    let segments = &self.vertical_line_segments.get(&(c.x as usize));
                    if let Some(segments) = segments {
                        for seg in segments.iter() {
                            assert!(seg.0.y < seg.1.y);
                            if seg.0.y < c.y && c.y < seg.1.y {
                                intersections += 1;
                                continue;
                            }

                            let corner_direction = if seg.0.y == c.y {
                                Direction::Down
                            } else if seg.1.y == c.y {
                                Direction::Up
                            } else {
                                continue;
                            };

                            if let Some(dir) = &last_corner_direction {
                                if *dir != corner_direction {
                                    intersections += 1;
                                }
                                last_corner_direction = None;
                            } else {
                                last_corner_direction = Some(corner_direction);
                            }
                        }
                    }
                }
                Direction::Up | Direction::Down => {
                    let segments = &self.horizontal_line_segments.get(&(c.y as usize));
                    if let Some(segments) = segments {
                        for seg in segments.iter() {
                            assert!(seg.0.x < seg.1.x);
                            if seg.0.x < c.x && c.x < seg.1.x {
                                intersections += 1;
                                continue;
                            }

                            let corner_direction = if seg.0.x == c.x {
                                Direction::Left
                            } else if seg.1.x == c.x {
                                Direction::Right
                            } else {
                                continue;
                            };

                            if let Some(dir) = &last_corner_direction {
                                if *dir != corner_direction {
                                    intersections += 1;
                                }
                                last_corner_direction = None;
                            } else {
                                last_corner_direction = Some(corner_direction);
                            }
                        }
                    }
                }
            };
        }

        assert_eq!(last_corner_direction, None);

        let result = intersections % 2 == 1;
        cache.insert(*coord, result);

        result
    }
}

struct BoxBorderIter {
    bounds: Bounds,
    side: Direction,
    current: Coordinate,
}

impl BoxBorderIter {
    fn new(a: &Coordinate, b: &Coordinate) -> Self {
        let x_min = a.x.min(b.x);
        let x_max = a.x.max(b.x);
        let y_min = a.y.min(b.y);
        let y_max = a.y.max(b.y);

        BoxBorderIter {
            bounds: Bounds {
                x_min,
                x_max,
                y_min,
                y_max,
            },
            side: Direction::Left,
            current: Coordinate { x: x_min, y: y_min },
        }
    }
}

impl Iterator for BoxBorderIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;

        match self.side {
            Direction::Left => {
                if self.current.y < self.bounds.y_max {
                    self.current.y += 1;
                } else {
                    self.side = Direction::Up;
                    self.current.x += 1;
                }
            }
            Direction::Up => {
                if self.current.x < self.bounds.x_max {
                    self.current.x += 1;
                } else {
                    self.side = Direction::Right;
                    self.current.y -= 1;
                }
            }
            Direction::Right => {
                if self.current.y > self.bounds.y_min {
                    self.current.y -= 1;
                } else {
                    self.side = Direction::Down;
                    self.current.x -= 1;
                }
            }
            Direction::Down => {
                if self.current.x > self.bounds.x_min {
                    self.current.x -= 1;
                } else {
                    return None;
                }
            }
        }

        Some(result)
    }
}

pub fn part1(input: &str) -> String {
    let tiles: Result<Vec<Coordinate>, ParseCoordinateError> = input
        .lines()
        .map(|line| line.parse::<Coordinate>())
        .collect();
    let tiles = match tiles {
        Ok(t) => t,
        Err(_) => return "Error parsing input".into(),
    };

    let mut area_entries: Vec<AreaEntry> = Vec::new();
    for (i, a) in tiles.iter().enumerate() {
        for (j, b) in tiles.iter().enumerate().skip(i + 1) {
            let dist = a.rectangle_area(b);
            area_entries.push(AreaEntry {
                tile_ids: (i, j),
                area: dist,
            });
        }
    }

    area_entries.iter().max().unwrap().area.to_string()
}

pub fn part2(input: &str) -> String {
    let tiles: Result<Vec<Coordinate>, ParseCoordinateError> = input
        .lines()
        .map(|line| line.parse::<Coordinate>())
        .collect();
    let tiles = match tiles {
        Ok(t) => t,
        Err(_) => return "Error parsing input".into(),
    };

    let mut area_entries: Vec<AreaEntry> = Vec::new();
    for (i, a) in tiles.iter().enumerate() {
        for (j, b) in tiles.iter().enumerate().skip(i + 1) {
            let dist = a.rectangle_area(b);
            area_entries.push(AreaEntry {
                tile_ids: (i, j),
                area: dist,
            });
        }
    }

    let raycaster = Raycaster::new(tiles);

    let mut cache: HashMap<Coordinate, bool> = HashMap::new();

    let mut max_heap = BinaryHeap::from(area_entries);
    loop {
        if let Some(entry) = max_heap.pop() {
            let is_valid_rectangle = BoxBorderIter::new(
                &raycaster.tiles[entry.tile_ids.0],
                &raycaster.tiles[entry.tile_ids.1],
            )
            .all(|coord| raycaster.is_enclosed(&coord, &mut cache));
            if is_valid_rectangle {
                return entry.area.to_string();
            }
        } else {
            unreachable!("No rectangle found");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_border_iter() {
        let a = Coordinate { x: 2, y: 3 };
        let b = Coordinate { x: 5, y: 6 };
        let mut iter = BoxBorderIter::new(&a, &b);
        let expected_coords = vec![
            Coordinate { x: 2, y: 3 },
            Coordinate { x: 2, y: 4 },
            Coordinate { x: 2, y: 5 },
            Coordinate { x: 2, y: 6 },
            Coordinate { x: 3, y: 6 },
            Coordinate { x: 4, y: 6 },
            Coordinate { x: 5, y: 6 },
            Coordinate { x: 5, y: 5 },
            Coordinate { x: 5, y: 4 },
            Coordinate { x: 5, y: 3 },
            Coordinate { x: 4, y: 3 },
            Coordinate { x: 3, y: 3 },
        ];
        for expected in expected_coords {
            let coord = iter.next().unwrap();
            assert_eq!(coord, expected);
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_part1() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
        assert_eq!(part1(input), "50");
    }

    #[test]
    fn test_part2() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
        assert_eq!(part2(input), "24");
    }
}
