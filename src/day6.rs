use std::time::Instant;
use std::{fs, io};

pub fn run() {
    println!("Day 6");

    // Open file
    let input = fs::read_to_string("inputs/day6.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split(",").collect();

    println!("Part 1 is {:?}", part1(&input, 80));
    println!("Part 2 is {:?}", part2(&input, 256));
}

fn part1(input: &Vec<&str>, days: u32) -> Result<i32, io::Error> {
    // New blank vector for storing all the fish
    let mut fish = Vec::new();
    // Add initial values to the vector
    for i in input {
        fish.push(i.parse::<u32>().unwrap());
    }

    // Cycle through each day
    for _ in 0..days {
        // Cycle through each fish
        for f in 0..fish.len() {
            // If the fish creates a new fish, reset its timer and create a new fish
            if fish[f] == 0 {
                fish[f] = 6;
                fish.push(8)
            }
            // If not, lower internal timer
            else {
                fish[f] -= 1;
            }
        }
    }

    Ok(fish.len() as i32)
}

fn part2(input: &Vec<&str>, days: u32) -> Result<u64, io::Error> {
    // Store fish in array by using tehir timer as an index
    let mut fish = [0 as u64; 9];
    // Initialise array
    for i in input {
        fish[i.parse::<usize>().unwrap()] += 1;
    }

    let mut fish_count = 0;

    // Cycle through each day
    for d in 0..days {
        // Start a timer for benchmarking
        let start = Instant::now();
        // Store how many new fish will be created
        let new_fish = fish[0];
        // Cycle through each fish and lower their timer by 1
        for f in 1..fish.len() {
            fish[f - 1] = fish[f];
        }
        // Create the new fish and move the parents to a timer of 8
        fish[6] += new_fish;
        fish[8] = new_fish;
        let duration = start.elapsed();

        // Count the fish
        fish_count = 0;
        for f in &fish {
            fish_count += f;
        }

        // Print debug information
        println!(
            "Day {} has a length of {} and took {:?}",
            d, fish_count, duration
        );
    }

    Ok(fish_count)
}
