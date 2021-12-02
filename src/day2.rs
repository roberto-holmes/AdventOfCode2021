use std::fs;
use std::io;
// use std::io::Result;

#[derive(Debug)]
struct Step {
    direction: String,
    magnitude: i32,
}

pub fn run() {
    println!("Day 2");
    println!("Part 1 is {:?}", part1());
}

fn part1() -> Result<i32, io::Error> {
    // Open file
    let input_str = fs::read_to_string("inputs/day2.txt")?;
    // Extract each line
    let input: Vec<&str> = input_str.split("\n").collect();
    // Prepare a vector to store data in structs
    let mut course: Vec<Step> = Vec::new();

    // Cycle through the lines of input
    for i in input {
        // Ignore empty lines
        if i != "" {
            // Separate each line into direction and value
            let splitted: Vec<&str> = i.split(" ").collect();
            let mut dir = splitted[0];
            let mut val = splitted[1].parse::<i32>().unwrap_or(0);
            // Simplify by making up = -down
            if dir == "up" {
                dir = "down";
                val = 0 - val;
            }
            //Add values to vector
            course.push(Step {
                direction: dir.to_string(),
                magnitude: val,
            });
        }
    }
    // Store coordinates
    let mut x = 0;
    let mut y = 0;
    // Cycle through each step to calculate overall coordinates
    for c in course {
        if c.direction == "down" {
            y += c.magnitude;
        } else if c.direction == "forward" {
            x += c.magnitude;
        } else {
            println!("Invalid direction");
        }
    }
    println!("({}, {}) result is", x, y);
    Ok(x * y)
}
