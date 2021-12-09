use colored::*;
use std::{fs, io};

pub fn run() {
    println!("Day 9");

    // Open file
    let input = fs::read_to_string("inputs/day9.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    // println!("Part 2 is {:?}", part2(&input));
}

fn part1(input: &Vec<&str>) -> Result<u32, io::Error> {
    let mut risk_level = 0;
    for row_num in 0..input.len() {
        let row: Vec<char> = input[row_num].chars().collect();
        for col_num in 0..row.len() {
            let current_value = row[col_num].to_digit(10).unwrap();
            // Array to store values of adjacent points in format top, bot, left, right
            let mut adjacent = [10; 4];
            // Top
            if row_num != 0 {
                adjacent[0] = input[row_num - 1].chars().collect::<Vec<char>>()[col_num]
                    .to_digit(10)
                    .unwrap();
            }
            // Bottom
            if row_num != input.len() - 1 {
                adjacent[1] = input[row_num + 1].chars().collect::<Vec<char>>()[col_num]
                    .to_digit(10)
                    .unwrap();
            }
            // Left
            if col_num != 0 {
                adjacent[2] = row[col_num - 1].to_digit(10).unwrap();
            }
            // Right
            if col_num != row.len() - 1 {
                adjacent[3] = row[col_num + 1].to_digit(10).unwrap();
            }

            // Check if we are at a low point
            if current_value < *adjacent.iter().min().unwrap() {
                risk_level += current_value + 1;
                // print!("{}", current_value.to_string().bright_magenta());
            } else {
                // print!("{}", current_value.to_string().black());
            }
        }
        // println!();
    }
    Ok(risk_level)
}
