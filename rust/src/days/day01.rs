fn parse_instruction(instruction: &str) -> (i32, i32) {
    let (dir_str, distance_str) = instruction.split_at(1);
    let direction = match dir_str {
        "L" => -1,
        "R" => 1,
        _ => panic!("Invalid direction, expected 'L' or 'R', got {}", &dir_str),
    };
    let distance: i32 = distance_str.parse().unwrap_or_else(|_| {
        panic!(
            "Failed to parse distance '{}' in instruction '{}'",
            distance_str, instruction
        )
    });

    (direction, distance)
}

pub fn part1(input: &str) -> String {
    let mut pos = 50;
    let mut answer = 0;

    for line in input.lines() {
        let (direction, distance) = parse_instruction(line.trim());
        pos += direction * distance;
        pos %= 100;
        if pos == 0 {
            answer += 1;
        }
    }

    answer.to_string()
}

pub fn part2(input: &str) -> String {
    let mut pos = 50;
    let mut answer = 0;

    for line in input.lines() {
        let (direction, distance) = parse_instruction(line.trim());
        answer += distance / 100;

        let old_pos = pos;
        pos += direction * (distance % 100);

        if old_pos != 0 && (pos <= 0 || pos >= 100) {
            answer += 1;
        }

        pos = pos.rem_euclid(100)
    }

    answer.to_string()
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
    fn test_part2() {
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

        assert_eq!(part2(input), "6");
    }
}
