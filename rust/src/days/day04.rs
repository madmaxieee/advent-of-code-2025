use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Paper,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
struct Coord {
    row: i32,
    col: i32,
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

    fn set_empty(&mut self, c: Coord) {
        self.cells[c.row as usize][c.col as usize] = Cell::Empty;
    }

    fn neighbors<'a>(&'a self, c: Coord) -> impl Iterator<Item = Cell> + 'a {
        let (rows, cols) = self.dimensions();
        let (r, k) = (c.row, c.col);

        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(move |(dx, dy)| (r + dx, k + dy))
        .filter(move |&(nx, ny)| nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32)
        .map(|(x, y)| self.cells[x as usize][y as usize])
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

pub fn part1(input: &str) -> String {
    let diagram = parse_diagram(input);
    let mut answer = 0;

    let (rows, cols) = diagram.dimensions();

    for (r, row) in diagram.cells.iter().enumerate().take(rows) {
        for (c, col) in row.iter().enumerate().take(cols) {
            match col {
                Cell::Empty => {}
                Cell::Paper => {
                    let n = diagram
                        .neighbors(Coord {
                            row: r as i32,
                            col: c as i32,
                        })
                        .filter(|cell| matches!(cell, Cell::Paper))
                        .count();
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
                        row: r as i32,
                        col: c as i32,
                    });
                }
            }
        }
    }

    loop {
        let mut to_remove: Vec<Coord> = vec![];

        for paper_coord in all_paper_coords.iter() {
            let n = diagram
                .neighbors(Coord {
                    row: paper_coord.row,
                    col: paper_coord.col,
                })
                .filter(|cell| matches!(cell, Cell::Paper))
                .count();
            if n < 4 {
                to_remove.push(*paper_coord);
            }
        }

        if to_remove.is_empty() {
            break;
        }

        answer += to_remove.len();

        for coord in to_remove.iter() {
            all_paper_coords.remove(coord);
            diagram.set_empty(*coord);
        }
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
