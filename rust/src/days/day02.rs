fn parse_range(s: &str) -> Option<(u64, u64)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return None;
    }
    let start = parts[0].parse::<u64>().ok()?;
    let end = parts[1].parse::<u64>().ok()?;
    Some((start, end))
}

fn num_digits(n: u64) -> u32 {
    let mut n = n;
    let mut l = 0;
    while n > 0 {
        n /= 10;
        l += 1;
    }
    l
}

fn first_chunk(n: u64, splits: u32) -> Option<u64> {
    let digit_length = num_digits(n);
    if !digit_length.is_multiple_of(splits) {
        return None;
    }
    let offset = 10_u64.pow(digit_length / splits);
    Some(n / offset.pow(splits - 1))
}

fn repeat_digits(n: u64, times: u32) -> u64 {
    let step = 10_u64.pow(num_digits(n));
    let mut result = 0;
    let mut offset = 1;
    for _ in 0..times {
        result += n * offset;
        offset *= step;
    }
    result
}

pub fn part1(input: &str) -> String {
    let mut answer: u64 = 0;

    for range_str in input.split(',') {
        let (min, max) = parse_range(range_str.trim())
            .unwrap_or_else(|| panic!("parse range failed: '{}'", range_str));

        let max = if num_digits(max).is_multiple_of(2) {
            max
        } else {
            10_u64.pow(num_digits(max) - 1) - 1
        };

        let mut top = first_chunk(max, 2).unwrap();
        let mut offset = 10_u64.pow(num_digits(max) / 2);

        while top * offset + top >= min {
            while top >= offset / 10 && (top * offset + top) >= min {
                if top * offset + top <= max {
                    answer += top * offset + top;
                }
                top -= 1;
            }
            top /= 10;
            offset /= 100;
        }
    }

    answer.to_string()
}

pub fn part2(input: &str) -> String {
    let mut answer: u64 = 0;

    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    let mut counted = std::collections::HashSet::<u64>::new();

    for range_str in input.split(',') {
        let (min, max) = parse_range(range_str.trim())
            .unwrap_or_else(|| panic!("parse range failed: '{}'", range_str));

        for times in primes {
            let max_digit_len = num_digits(max);
            let max = if max_digit_len.is_multiple_of(times) {
                max
            } else {
                10_u64.pow(max_digit_len - (max_digit_len % times)) - 1
            };

            let mut top = first_chunk(max, times).unwrap();
            let mut offset = 10_u64.pow(num_digits(max) / times);

            let mut val = repeat_digits(top, times);
            while val >= min {
                while top >= offset / 10 && val >= min {
                    if val <= max && !counted.contains(&val) {
                        counted.insert(val);
                        answer += val;
                    }
                    top -= 1;
                    val = repeat_digits(top, times);
                }
                top /= 10;
                offset /= 10_u64.pow(times);
                val = repeat_digits(top, times);
            }
        }
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 0);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(99), 2);
        assert_eq!(num_digits(234), 3);
        assert_eq!(num_digits(1000), 4);
    }

    #[test]
    fn test_repeat_digits() {
        assert_eq!(repeat_digits(5, 3), 555);
        assert_eq!(repeat_digits(12, 2), 1212);
        assert_eq!(repeat_digits(10, 4), 10101010);
    }

    #[test]
    fn test_part1() {
        let input = r#"48-84"#;
        assert_eq!(part1(input), (55 + 66 + 77).to_string());
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
        assert_eq!(part1(input), "1227775554");
    }

    #[test]
    fn test_part2() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
        assert_eq!(part2(input), "4174379265");
    }
}
