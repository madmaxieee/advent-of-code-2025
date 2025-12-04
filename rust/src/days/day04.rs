use std::collections::BTreeSet;

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Paper,
}

type Cells = Vec<Vec<Cell>>;

struct Diagram {
    cells: Cells,
}

impl Diagram {
    #[allow(dead_code)]
    fn print(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Paper => print!("@"),
                }
            }
            println!();
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        // rows, cols
        (self.cells.len(), self.cells[0].len())
    }

    fn get(&self, c: &Coord) -> Cell {
        self.cells[c.x as usize][c.y as usize].clone()
    }

    fn set_empty(&mut self, c: &Coord) {
        self.cells[c.x as usize][c.y as usize] = Cell::Empty;
    }
}

fn parse_diagram(input: &str) -> Diagram {
    let cells: Cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Paper,
                    _ => panic!("unexpected character"),
                })
                .collect()
        })
        .collect();
    Diagram { cells }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
}

fn get_adjacent(row: usize, col: usize, c: Coord) -> Vec<Coord> {
    let mut result = vec![];

    let row = row as i32;
    let col = col as i32;

    for (dx, dy) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        if (0..row).contains(&(c.x + dx)) && (0..col).contains(&(c.y + dy)) {
            result.push(Coord {
                x: c.x + dx,
                y: c.y + dy,
            });
        }
    }

    result
}

pub fn part1(input: &str) -> String {
    let diagram = parse_diagram(input);
    let mut answer = 0;

    let (rows, cols) = diagram.dimensions();

    for (r, row) in diagram.cells.iter().enumerate().take(rows) {
        for (c, col) in row.iter().enumerate().take(cols) {
            match col {
                Cell::Empty => {}
                Cell::Paper => {
                    let adjacent = get_adjacent(
                        rows,
                        cols,
                        Coord {
                            x: r as i32,
                            y: c as i32,
                        },
                    );
                    let n: u32 = adjacent
                        .iter()
                        .map(|coord| match diagram.get(coord) {
                            Cell::Empty => 0,
                            Cell::Paper => 1,
                        })
                        .sum();
                    if n < 4 {
                        answer += 1;
                    }
                }
            }
        }
    }

    answer.to_string()
}

pub fn part2(input: &str) -> String {
    let mut diagram = parse_diagram(input);
    let mut answer = 0;

    let mut all_paper_coords = BTreeSet::<Coord>::new();

    let (rows, cols) = diagram.dimensions();

    for (r, row) in diagram.cells.iter().enumerate().take(rows) {
        for (c, col) in row.iter().enumerate().take(cols) {
            match col {
                Cell::Empty => {}
                Cell::Paper => {
                    all_paper_coords.insert(Coord {
                        x: r as i32,
                        y: c as i32,
                    });
                }
            }
        }
    }

    loop {
        let mut to_remove: Vec<Coord> = vec![];

        for paper_coord in all_paper_coords.iter() {
            let adjacent = get_adjacent(
                rows,
                cols,
                Coord {
                    x: paper_coord.x,
                    y: paper_coord.y,
                },
            );
            let n: u32 = adjacent
                .iter()
                .map(|coord| match diagram.get(coord) {
                    Cell::Empty => 0,
                    Cell::Paper => 1,
                })
                .sum();
            if n < 4 {
                to_remove.push(paper_coord.clone());
            }
        }

        if to_remove.is_empty() {
            break;
        }

        answer += to_remove.len();

        for coord in to_remove.iter() {
            all_paper_coords.remove(coord);
            diagram.set_empty(coord);
        }
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adjacent() {
        assert_eq!(
            get_adjacent(10, 10, Coord { x: 1, y: 1 }),
            vec![
                Coord { x: 0, y: 0 },
                Coord { x: 0, y: 1 },
                Coord { x: 0, y: 2 },
                Coord { x: 1, y: 0 },
                Coord { x: 1, y: 2 },
                Coord { x: 2, y: 0 },
                Coord { x: 2, y: 1 },
                Coord { x: 2, y: 2 },
            ]
        );
        assert_eq!(
            get_adjacent(10, 10, Coord { x: 0, y: 0 }),
            vec![
                Coord { x: 0, y: 1 },
                Coord { x: 1, y: 0 },
                Coord { x: 1, y: 1 },
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;
        assert_eq!(part1(input), "13");
    }

    #[test]
    fn test_part2() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;
        assert_eq!(part2(input), "43");
    }
}
