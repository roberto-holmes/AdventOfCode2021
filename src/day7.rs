use std::time::Instant;
use std::{fs, io};

pub fn run() {
    println!("Day 7");

    // Open file
    let input = fs::read_to_string("inputs/day7.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split(",").collect();
    // Create vector of u32 instead of &str
    let mut input_num = Vec::new();
    for i in &input {
        input_num.push(i.parse::<u32>().expect("Invalid number"));
    }

    println!("Part 1 is {:?}", part1(&input_num));
    println!("Part 2 is {:?}", part2(&input_num));
}

fn part1(input: &Vec<u32>) -> Result<i32, io::Error> {
    // Create values for storing the current best
    let mut best_pos = 0;
    let mut best_cost = 0;
    // Cycle through each potential position
    for pos in 0..input.len() {
        let mut cost: i32 = 0;
        // Calculate the cost if each value were to go to that position
        for i in input {
            cost += (*i as i32 - pos as i32).abs();
        }
        // If the cost is a personal best, store it
        if best_cost == 0 || cost < best_cost {
            best_cost = cost;
            best_pos = pos;
        }
    }
    println!("Optimal alignment position is {}", best_pos);
    Ok(best_cost)
}

fn part2(input: &Vec<u32>) -> Result<i32, io::Error> {
    // Create values for storing the current best
    let mut best_pos = 0;
    let mut best_cost = 0;
    let mut last_cost = 0;
    // Start a timer for benchmarking
    let start = Instant::now();
    // Cycle through each potential position
    for pos in 0..input.len() {
        let mut cost: i32 = 0;
        // Calculate the cost if each value were to go to that position
        for i in input {
            // Implement the new cost function
            for step in 0..(*i as i32 - pos as i32).abs() + 1 {
                cost += step;
            }
        }
        // println!("Position {} has a cost of {}", pos, cost);
        // If the cost is a personal best, store it
        if best_cost == 0 || cost < best_cost {
            best_cost = cost;
            best_pos = pos;
        }
        // Avoid unecessary calculation if we already have the answer
        if cost < last_cost || last_cost == 0 {
            last_cost = cost
        } else {
            break;
        }
    }
    println!("Part 2 completed in {:?}", start.elapsed());
    println!("Optimal alignment position is {}", best_pos);
    Ok(best_cost)
}
