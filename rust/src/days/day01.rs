pub fn part1(_input: &str) -> String {
    "".to_string()
}

pub fn part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2() {}
}
