fn parse_bank(s: &str) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn part1(input: &str) -> String {
    let mut answer = 0;

    for line in input.lines() {
        let bank = parse_bank(line);
        let mut max_first_digit: u32 = 0;
        let mut first_digit_idx: usize = 0;
        for (i, n) in bank[0..bank.len() - 1].iter().enumerate() {
            if *n > max_first_digit {
                max_first_digit = *n;
                first_digit_idx = i;
            }
        }
        let mut max_second_digit = 0;
        for n in bank[(first_digit_idx + 1)..].iter() {
            if n > &max_second_digit {
                max_second_digit = *n;
            }
        }
        let joltage = max_first_digit * 10 + max_second_digit;
        answer += joltage
    }

    answer.to_string()
}

pub fn part2(input: &str) -> String {
    let mut answer = 0;

    for line in input.lines() {
        let bank = parse_bank(line);
        let mut skip_amount: usize = 0;
        let mut joltage: u64 = 0;
        for offset in (0..12).rev() {
            let mut max_digit: u32 = 0;
            for (i, n) in bank[..(bank.len() - offset)]
                .iter()
                .enumerate()
                .skip(skip_amount)
            {
                if *n > max_digit {
                    max_digit = *n;
                    skip_amount = i + 1;
                }
            }
            joltage += (max_digit as u64) * 10_u64.pow(offset.try_into().unwrap());
        }
        answer += joltage;
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bank() {
        assert_eq!(parse_bank("1"), vec![1]);
        assert_eq!(parse_bank("123"), vec![1, 2, 3]);
    }

    #[test]
    fn test_part1() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(part1(input), "357");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("987654321111111"), "987654321111");
        assert_eq!(part2("811111111111119"), "811111111119");
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(part2(input), "3121910778619");
    }
}
