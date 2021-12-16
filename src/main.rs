mod day0;
mod day1;
mod day10;
mod day11;
mod day13;
mod day14;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Get the day number to be run from the arguments
    let current_day: i32 = args[1].parse().unwrap();

    // Run the relevant day
    match current_day {
        16 => day16::run(),
        14 => day14::run(),
        13 => day13::run(),
        11 => day11::run(),
        10 => day10::run(),
        9 => day9::run(),
        8 => day8::run(),
        7 => day7::run(),
        6 => day6::run(),
        5 => day5::run(),
        4 => day4::run(),
        3 => day3::run(),
        2 => day2::run(),
        1 => day1::run(),
        0 => day0::run(),
        _ => println!("Invalid day"),
    }
}
