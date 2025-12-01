mod days;

use days::get_day;
use std::env;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: cargo run <day>".into());
    }

    let day = if let Ok(day) = args[1].parse() {
        day
    } else {
        return Err(format!("Day must be a number, got '{}'", args[1]).into());
    };

    if !(1..=25).contains(&day) {
        return Err("Day must be between 1 and 25".into());
    }

    let input_path = format!("../inputs/day{:02}.txt", day);
    let input_path = fs::canonicalize(input_path)?;
    let input_path = input_path.as_path();

    let input = if let Ok(input) = fs::read_to_string(input_path) {
        input
    } else {
        return Err(format!("Could not read file {}", input_path.to_str().unwrap()).into());
    };

    let (part1, part2) = get_day(day);

    println!("Part 1:");
    let start = Instant::now();
    println!("{}", part1(&input));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!();

    println!("Part 2:");
    let start = Instant::now();
    println!("{}", part2(&input));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
