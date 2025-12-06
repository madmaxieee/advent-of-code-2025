#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Unknown operation"),
        }
    }
}

impl From<char> for Operation {
    fn from(c: char) -> Self {
        match c {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => panic!("Unknown operation"),
        }
    }
}

struct Problem {
    numbers: Vec<u64>,
    operation: Option<Operation>,
}

impl Problem {
    fn new() -> Self {
        Problem {
            numbers: Vec::new(),
            operation: None,
        }
    }

    fn solve(&self) -> u64 {
        match self.operation {
            Some(Operation::Add) => self.numbers.iter().sum(),
            Some(Operation::Multiply) => self.numbers.iter().product(),
            None => panic!("No operation specified"),
        }
    }
}

fn parse_problems_1(input: &str) -> Vec<Problem> {
    let num_problems = input.lines().next().unwrap().split_whitespace().count();
    let mut problems: Vec<Problem> = Vec::with_capacity(num_problems);

    for _ in 0..num_problems {
        problems.push(Problem::new());
    }

    for line in input.lines() {
        for (i, tok) in line.split_whitespace().enumerate() {
            if let Ok(n) = tok.parse::<u64>() {
                problems[i].numbers.push(n);
            } else {
                let op = Some(tok.into());
                problems[i].operation = op;
            }
        }
    }

    problems
}

fn parse_problems_2(input: &str) -> Vec<Problem> {
    let last_line = input.lines().last().unwrap();
    let mut problems: Vec<Problem> = Vec::with_capacity(last_line.split_whitespace().count());

    let mut op_pos: Vec<usize> = last_line
        .chars()
        .enumerate()
        .filter(|&(_, c)| c != ' ')
        .map(|(i, _)| i)
        .collect();

    let last_line_chars: Vec<char> = last_line.chars().collect();
    for i in &op_pos {
        problems.push(Problem {
            numbers: Vec::new(),
            operation: Some(last_line_chars[*i].into()),
        });
    }

    op_pos.push(last_line.len() + 1);
    let column_ranges: Vec<(usize, usize)> = op_pos.windows(2).map(|w| (w[0], w[1] - 2)).collect();

    let lines_reversed: Vec<&str> = input.lines().rev().skip(1).collect();

    for (i, (start, end)) in column_ranges.iter().enumerate() {
        let mut numbers: Vec<u64> = Vec::new();

        for i in (*start..=*end).rev() {
            let mut n = 0;
            let mut scale = 1;
            for line in &lines_reversed {
                if let Some(c) = (line.as_bytes()[i] as char).to_digit(10) {
                    n += c as u64 * scale;
                    scale *= 10;
                }
            }
            numbers.push(n);
        }

        problems[i].numbers = numbers;
    }

    problems
}
pub fn part1(input: &str) -> String {
    let problems = parse_problems_1(input);

    problems
        .iter()
        .map(|problem| problem.solve())
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let problems = parse_problems_2(input);

    problems
        .iter()
        .map(|problem| problem.solve())
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        assert_eq!(part1(input), "4277556");
    }

    #[test]
    fn test_part2() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        assert_eq!(part2(input), "3263827");
    }
}
