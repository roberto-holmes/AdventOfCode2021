use std::{fs, io};

pub fn run() {
    println!("Day 6");

    // Open file
    let input = fs::read_to_string("inputs/day6.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split(",").collect();

    println!("Part 1 is {:?}", part1(&input, 80));
    // println!("Part 2 is {:?}", part2());
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
