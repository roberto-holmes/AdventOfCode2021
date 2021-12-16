use colored::*;
use std::{fs, io};

// Store folding instructions by the function of the line (e.g. x=3)
#[derive(Debug)]
struct Fold {
    direction: char,
    position: usize,
}

pub fn run() {
    println!("Day 13");

    // Open file
    let input = fs::read_to_string("inputs/day13.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    println!("Part 2 is {:?}", part2(&input));
}

fn part1(input: &Vec<&str>) -> Result<u32, io::Error> {
    let mut instructions = Vec::new();
    let mut reached_instructions = false;
    let mut page = Vec::new();
    for y in 0..input.len() {
        let row = input[y];
        // Parse instructions
        if reached_instructions {
            let command: Vec<&str> = row.split(" ").collect();
            let line: Vec<&str> = command[2].split("=").collect();
            instructions.push(Fold {
                direction: line[0].chars().next().unwrap(),
                position: line[1].parse::<usize>().unwrap(),
            });
        }
        // Figure out when the second section of the input has been reached
        else if row == "" {
            reached_instructions = true;
        }
        // Deal with dots on the page
        else {
            let command: Vec<&str> = row.split(",").collect();
            let x = command[0].parse::<usize>().unwrap();
            let y = command[1].parse::<usize>().unwrap();

            // Add blank spaces up to the y coordinate
            while page.len() <= y {
                page.push(Vec::new());
            }
            // Add blank spaces up to the x
            while page[y].len() <= x {
                page[y].push(0);
            }
            // Add the dot
            page[y][x] += 1;
        }
    }

    // Perform fold
    if instructions[0].direction == 'x' {
        for y in 0..page.len() {
            for x in (instructions[0].position..page[y].len()).rev() {
                page[y][2 * instructions[0].position - x] += page[y][x];
                // Remove part of the page that has een folded over
                page[y].remove(x);
            }
        }
    } else if instructions[0].direction == 'y' {
        for y in (instructions[0].position..page.len()).rev() {
            for x in 0..page[y].len() {
                // Add extra blank x if they are needed
                while page[2 * instructions[0].position - y].len() <= x {
                    page[2 * instructions[0].position - y].push(0);
                }
                page[2 * instructions[0].position - y][x] += page[y][x];
            }
            // Remove part of the page that has een folded over
            page.remove(y);
        }
    }

    // Count (and display) dots
    let mut dot_count = 0;
    for y in &page {
        for x in y {
            if x > &0 {
                print!("{}", x.to_string().yellow());
                dot_count += 1;
            } else {
                print!("{}", x.to_string().blue());
            }
        }
        println!();
    }
    // println!("{:?}", page);
    // println!("{:?}", instructions);
    Ok(dot_count)
}

fn part2(input: &Vec<&str>) -> Result<u32, io::Error> {
    let mut instructions = Vec::new();
    let mut reached_instructions = false;
    let mut page = Vec::new();
    for y in 0..input.len() {
        let row = input[y];
        // Parse instructions
        if reached_instructions {
            let command: Vec<&str> = row.split(" ").collect();
            let line: Vec<&str> = command[2].split("=").collect();
            instructions.push(Fold {
                direction: line[0].chars().next().unwrap(),
                position: line[1].parse::<usize>().unwrap(),
            });
        }
        // Figure out when the second section of the input has been reached
        else if row == "" {
            reached_instructions = true;
        }
        // Deal with dots on the page
        else {
            let command: Vec<&str> = row.split(",").collect();
            let x = command[0].parse::<usize>().unwrap();
            let y = command[1].parse::<usize>().unwrap();

            // Add blank spaces up to the y coordinate
            while page.len() <= y {
                page.push(Vec::new());
            }
            // Add blank spaces up to the x
            while page[y].len() <= x {
                page[y].push(0);
            }
            // Add the dot
            page[y][x] += 1;
        }
    }

    // Perform folds
    for instruction in instructions {
        if instruction.direction == 'x' {
            for y in 0..page.len() {
                for x in (instruction.position..page[y].len()).rev() {
                    page[y][2 * instruction.position - x] += page[y][x];
                    // Remove part of the page that has een folded over
                    page[y].remove(x);
                }
            }
        } else if instruction.direction == 'y' {
            for y in (instruction.position..page.len()).rev() {
                for x in 0..page[y].len() {
                    // Add extra blank x if they are needed
                    while page[2 * instruction.position - y].len() <= x {
                        page[2 * instruction.position - y].push(0);
                    }
                    page[2 * instruction.position - y][x] += page[y][x];
                }
                // Remove part of the page that has een folded over
                page.remove(y);
            }
        }
    }

    // Count (and display) dots
    let mut dot_count = 0;
    for y in &page {
        for x in y {
            if x > &0 {
                print!("{}", "#".yellow());
                dot_count += 1;
            } else {
                print!("{}", "-".blue());
            }
        }
        println!();
    }
    Ok(dot_count)
}
