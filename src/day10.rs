use colored::*;
use std::{fs, io};

pub fn run() {
    println!("Day 10");

    // Open file
    let input = fs::read_to_string("inputs/day10.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    // println!("Part 2 is {:?}", part2(&input));
}

fn part1(input: &Vec<&str>) -> Result<u32, io::Error> {
    let mut answer = 0;
    for line in input {
        // Store active chunks
        let mut brackets = Vec::new();

        for c in line.chars() {
            // Check if a chunk is starting
            if c == '(' || c == '[' || c == '{' || c == '<' {
                // Add it to active chunks
                brackets.push(c);
                print!("{}", c.to_string().blue());
            } else {
                // Check if the chunk is being closed with the right character
                print!("{}", c.to_string().yellow());
                let last = *brackets.last().unwrap();
                if c == ')' {
                    // If it is a valid
                    if last == '(' {
                        // Close the chunk by removing the opening character
                        brackets.pop();
                    } else {
                        // End the line and add up the cost of the character
                        answer += 3;
                        break;
                    }
                } else if c == ']' {
                    if last == '[' {
                        brackets.pop();
                    } else {
                        answer += 57;
                        break;
                    }
                } else if c == '}' {
                    if last == '{' {
                        brackets.pop();
                    } else {
                        answer += 1197;
                        break;
                    }
                } else if c == '>' {
                    if last == '<' {
                        brackets.pop();
                    } else {
                        answer += 25137;
                        break;
                    }
                } else {
                    panic!("{} is an unexpected character", c);
                }
            }
        }
        println!();
    }
    Ok(answer)
}
