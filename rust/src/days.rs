pub type DayFunction = fn(&str) -> String;

pub fn noop(_: &str) -> String {
    "Not implemented".to_string()
}

macro_rules! days {
    ($($day_num:expr => $day_mod:ident),* $(,)?) => {
        $(
            pub mod $day_mod;
        )*

        pub fn get_day(day: u8) -> (DayFunction, DayFunction) {
            match day {
                $(
                    $day_num => ($day_mod::part1, $day_mod::part2),
                )*
                _ => (noop, noop),
            }
        }
    };
}

days!(
    1 => day01,
    2 => day02,
    3 => day03,
    4 => day04,
    5 => day05,
    6 => day06,
    7 => day07,
    8 => day08,
    9 => day09,
);
