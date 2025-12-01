mod day01;

pub fn noop(_: &str) -> String {
    "Not implemented".to_string()
}

pub type DayFunction = fn(&str) -> String;

pub fn get_day(day: u8) -> (DayFunction, DayFunction) {
    match day {
        1 => (day01::part1, day01::part2),
        _ => (noop, noop),
    }
}
