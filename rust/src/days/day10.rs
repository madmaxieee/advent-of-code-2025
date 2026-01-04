use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    ops::BitXor,
    str::FromStr,
};

use crate::utils::itertools::CombinationIter;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct BitSet {
    data: u32,
    size: usize,
}

#[allow(dead_code)]
impl BitSet {
    fn new_empty(size: usize) -> Self {
        BitSet { data: 0, size }
    }

    fn from_indices(indices: &[usize], size: usize) -> Self {
        BitSet {
            data: indices.iter().fold(0, |acc, &i| acc | (1 << i)),
            size,
        }
    }

    fn set(&mut self, index: usize) {
        self.data |= 1 << index;
    }

    fn get(&self, index: usize) -> bool {
        (self.data & (1 << index)) != 0
    }
}

impl BitXor for &BitSet {
    type Output = BitSet;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitSet {
            data: self.data ^ rhs.data,
            size: self.size,
        }
    }
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        for i in 0..self.size {
            if self.get(i) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push(']');
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Counter {
    counts: Vec<u32>,
}

impl Counter {
    fn new(size: usize) -> Self {
        Counter {
            counts: vec![0; size],
        }
    }

    fn from_vec(counts: Vec<u32>) -> Self {
        Counter { counts }
    }

    fn increment(&mut self, indices: &[usize]) {
        for &index in indices {
            self.counts[index] += 1;
        }
    }

    fn get(&self, index: usize) -> u32 {
        self.counts[index]
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Machine {
    light: BitSet,
    button_bitsets: Vec<BitSet>,
    joltage: Counter,
    button_indices: Vec<Vec<usize>>,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut iter = line.split_whitespace();

        let light_str = iter.next().unwrap();
        let num_lights = light_str.len() - 2;
        let light = BitSet::from_indices(
            &light_str[1..light_str.len() - 1]
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
                .collect::<Vec<usize>>(),
            num_lights,
        );

        let button_indices = iter
            .by_ref()
            .take_while(|s| s.starts_with('('))
            .map(|s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let button_bitsets = button_indices
            .iter()
            .map(|s| BitSet::from_indices(s, light.size))
            .collect::<Vec<BitSet>>();

        let joltage_str = line.split_whitespace().last().unwrap();
        let joltage = joltage_str[1..joltage_str.len() - 1]
            .split(',')
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let joltage_counter = Counter::from_vec(joltage);

        Ok(Machine {
            light,
            button_bitsets,
            button_indices,
            joltage: joltage_counter,
        })
    }
}

impl Machine {
    fn solve_part1(&self) -> usize {
        let mut cache: HashMap<BitSet, Vec<usize>> = HashMap::new();
        cache.insert(BitSet::new_empty(self.light.size), vec![]);

        for n in 1..=self.button_bitsets.len() {
            let mut min_buttons = usize::MAX;
            for combination in CombinationIter::new(&self.button_bitsets, n) {
                let combo_bitset = combination
                    .iter()
                    .fold(BitSet::new_empty(self.light.size), |acc, btn| &acc ^ btn);

                let combo_indices: Vec<usize> = combination
                    .iter()
                    .map(|btn| self.button_bitsets.iter().position(|b| b == btn).unwrap())
                    .collect();

                if let Some(indices) = cache.get(&(&self.light ^ &combo_bitset)) {
                    min_buttons = min_buttons.min(indices.len() + combo_indices.len());
                } else {
                    cache.entry(combo_bitset).or_insert(combo_indices);
                }
            }
            if min_buttons != usize::MAX {
                return min_buttons;
            }
        }

        0
    }

    fn solve_part2(&self) -> usize {
        #[derive(Debug, PartialEq, Eq)]
        struct State {
            cost: usize,
            estimated_total_cost: usize,
            counter: Counter,
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other
                    .estimated_total_cost
                    .cmp(&self.estimated_total_cost)
                    .then_with(|| other.cost.cmp(&self.cost))
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        // Calculate max weight of any button for the heuristic
        let max_button_weight = self
            .button_indices
            .iter()
            .map(|btn| btn.len())
            .max()
            .unwrap_or(1);

        let initial_counter = Counter::new(self.light.size);

        // Calculate initial heuristic
        let remaining: usize = self
            .joltage
            .counts
            .iter()
            .zip(initial_counter.counts.iter())
            .map(|(target, current)| (target - current) as usize)
            .sum();

        let heuristic = remaining.div_ceil(max_button_weight);

        pq.push(State {
            cost: 0,
            estimated_total_cost: heuristic,
            counter: initial_counter.clone(),
        });

        visited.insert(initial_counter);

        while let Some(State { cost, counter, .. }) = pq.pop() {
            if counter == self.joltage {
                return cost;
            }

            for btn in &self.button_indices {
                let mut new_counter = counter.clone();
                new_counter.increment(btn);

                // Check constraints
                let mut valid = true;
                let mut remaining_sum = 0;

                for (i, &count) in new_counter.counts.iter().enumerate() {
                    let target = self.joltage.get(i);
                    if count > target {
                        valid = false;
                        break;
                    }
                    remaining_sum += (target - count) as usize;
                }

                if valid && visited.insert(new_counter.clone()) {
                    let heuristic = remaining_sum.div_ceil(max_button_weight);
                    pq.push(State {
                        cost: cost + 1,
                        estimated_total_cost: cost + 1 + heuristic,
                        counter: new_counter,
                    });
                }
            }
        }

        0
    }
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .map(|machine| machine.solve_part1())
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .map(|machine| machine.solve_part2())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"#;
        assert_eq!(part1(input), "2");
    }

    #[test]
    fn test_part1_2() {
        let input = r#"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"#;
        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part1_3() {
        let input = r#"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        assert_eq!(part1(input), "2");
    }

    #[test]
    fn test_part1_all() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        assert_eq!(part1(input), "7");
    }

    #[test]
    fn test_part2_1() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"#;
        assert_eq!(part2(input), "10");
    }

    #[test]
    fn test_part2_2() {
        let input = r#"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"#;
        assert_eq!(part2(input), "12");
    }

    #[test]
    fn test_part2_3() {
        let input = r#"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        assert_eq!(part2(input), "11");
    }

    #[test]
    fn test_part2_all() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        assert_eq!(part2(input), "33");
    }
}
