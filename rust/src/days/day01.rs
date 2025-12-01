pub fn part1(input: &str) -> String {
    let mut pos = 50;
    let mut answer = 0;

    for line in input.lines() {
        let instruction = line.trim();
        let direction = match &instruction[0..1] {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };
        let distance: i32 = instruction[1..].parse().unwrap();
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
        let instruction = line.trim();
        let direction = match &instruction[0..1] {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };

        let distance: i32 = instruction[1..].parse().unwrap();
        answer += distance / 100;
        let real_distance = distance % 100;

        let old_pos = pos;
        pos += direction * real_distance;

        if old_pos != 0 && !(1..=99).contains(&pos) {
            answer += 1;
        }

        pos = (pos + 100) % 100;
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
