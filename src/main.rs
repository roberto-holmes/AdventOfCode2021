mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Get the day number to be run from the arguments
    let current_day: i32 = args[1].parse().unwrap();

    // Run the relevant day
    match current_day {
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
