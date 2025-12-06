struct Database {
    fresh_ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

fn parse_database(input: &str) -> Database {
    let mut fresh_ranges = vec![];
    let mut ingredients = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        match line.split_once('-') {
            Some((start, end)) => {
                let start: u64 = start.parse().unwrap();
                let end: u64 = end.parse().unwrap();
                fresh_ranges.push((start, end));
            }
            None => {
                if let Ok(ingredient) = line.parse() {
                    ingredients.push(ingredient);
                }
            }
        }
    }

    Database {
        fresh_ranges,
        ingredients,
    }
}

pub fn part1(input: &str) -> String {
    let db = parse_database(input);
    let mut answer = 0;

    for ingredient in db.ingredients {
        if db
            .fresh_ranges
            .iter()
            .any(|(start, end)| (*start..=*end).contains(&ingredient))
        {
            answer += 1;
        }
    }

    answer.to_string()
}

pub fn part2(input: &str) -> String {
    let mut db = parse_database(input);

    db.fresh_ranges
        .sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut merged_ranges: Vec<(u64, u64)> = vec![];
    for range in db.fresh_ranges {
        if let Some(last) = merged_ranges.last_mut()
            && range.0 <= last.1 + 1
        {
            last.1 = last.1.max(range.1);
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        assert_eq!(part2(input), "14");
    }
}
