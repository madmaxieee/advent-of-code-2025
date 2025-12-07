use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Diagram {
    beam_start: (usize, usize),
    splitters: Vec<Vec<usize>>,
}

impl FromStr for Diagram {
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut beam_start = None;
        let mut splitters = vec![vec![]; input.lines().next().unwrap().chars().count()];

        for (i, line) in input.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    beam_start = Some((i, j));
                } else if ch == '^' {
                    splitters[j].push(i);
                }
            }
        }

        Ok(Diagram {
            beam_start: beam_start.unwrap(),
            splitters,
        })
    }

    type Err = ();
}

impl Diagram {
    fn find_splitter_below(&self, beam: (usize, usize)) -> Option<(usize, usize)> {
        let (r, c) = beam;
        match self.splitters[c].binary_search(&r) {
            Ok(_) => {
                panic!("Beam cannot be on a splitter at {:?}", beam);
            }
            Err(idx) => {
                if idx < self.splitters[c].len() {
                    Some((self.splitters[c][idx], c))
                } else {
                    None
                }
            }
        }
    }
}

pub fn part1(input: &str) -> String {
    let diagram: Diagram = input.parse().unwrap();
    let mut beams = vec![diagram.beam_start];
    let mut splitter_hit: HashSet<(usize, usize)> = HashSet::new();
    let mut tested_beams: HashSet<(usize, usize)> = HashSet::new();

    while let Some(beam) = beams.pop() {
        if tested_beams.contains(&beam) {
            continue;
        }

        if let Some(splitter) = diagram.find_splitter_below(beam) {
            splitter_hit.insert(splitter);
            let (r, c) = splitter;
            if c > 0 {
                beams.push((r, c - 1));
            }
            if c + 1 < diagram.splitters.len() {
                beams.push((r, c + 1));
            }
        }

        tested_beams.insert(beam);
    }

    splitter_hit.len().to_string()
}

fn timelines(
    cache: &mut HashMap<(usize, usize), usize>,
    diagram: &Diagram,
    beam: (usize, usize),
) -> usize {
    if let Some(count) = cache.get(&beam) {
        return *count;
    }

    let count = if let Some((splitter_row, c)) = diagram.find_splitter_below(beam) {
        let mut count = 0;
        if c > 0 {
            count += timelines(cache, diagram, (splitter_row, c - 1));
        }
        if c + 1 < diagram.splitters.len() {
            count += timelines(cache, diagram, (splitter_row, c + 1));
        }
        count
    } else {
        1
    };

    cache.insert(beam, count);

    count
}

pub fn part2(input: &str) -> String {
    let diagram: Diagram = input.parse().unwrap();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    timelines(&mut cache, &diagram, diagram.beam_start).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        assert_eq!(part1(input), "21");
    }

    #[test]
    fn test_part2() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        assert_eq!(part2(input), "40");
    }
}
