mod day0;

use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Get the day number to be run from the arguments
    let current_day: i32 = args[1].parse().unwrap();

    // Run the relevant day
    match current_day {
        0 => day0::run(),
        _ => println!("Invalid day"),
    }
}
