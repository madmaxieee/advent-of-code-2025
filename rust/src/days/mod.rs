mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn noop(_: &str) -> String {
    "Not implemented".to_string()
}

pub type DayFunction = fn(&str) -> String;

pub fn get_day(day: u8) -> (DayFunction, DayFunction) {
    match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),
        6 => (day06::part1, day06::part2),
        _ => (noop, noop),
    }
}
