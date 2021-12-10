use colored::*;
use std::{fs, io};

pub fn run() {
    println!("Day 10");

    // Open file
    let input = fs::read_to_string("inputs/day10.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    println!("Part 2 is {:?}", part2(&input));
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

fn part2(input: &Vec<&str>) -> Result<u64, io::Error> {
    let mut answer: Vec<u64> = Vec::new();
    for line_num in 0..input.len() {
        answer.push(0);
        let line = input[line_num];
        // Store active chunks
        let mut brackets = Vec::new();
        let line_chars: Vec<char> = line.chars().collect();
        for i in 0..line_chars.len() {
            let c = line_chars[i];
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
                        // Corrupt line
                        print!(" corrupted line");
                        break;
                    }
                } else if c == ']' {
                    if last == '[' {
                        brackets.pop();
                    } else {
                        // Corrupt line
                        print!(" corrupted line");
                        break;
                    }
                } else if c == '}' {
                    if last == '{' {
                        brackets.pop();
                    } else {
                        // Corrupt line
                        print!(" corrupted line");
                        break;
                    }
                } else if c == '>' {
                    if last == '<' {
                        brackets.pop();
                    } else {
                        // Corrupt line
                        print!(" corrupted line");
                        break;
                    }
                } else {
                    panic!("{} is an unexpected character", c);
                }
            }
            // If the end of the line is reached and there are still active chunks
            if i == line_chars.len() - 1 && brackets.len() != 0 {
                // Calculate the cost of closing each chunk
                for j in (0..brackets.len()).rev() {
                    answer[line_num] *= 5;
                    match brackets[j] {
                        '(' => answer[line_num] += 1,
                        '[' => answer[line_num] += 2,
                        '{' => answer[line_num] += 3,
                        '<' => answer[line_num] += 4,
                        _ => panic!("Invalid character in brackets vector"),
                    }
                }
            }
        }
        println!();
    }
    // Process the answer by getting the median non zero value
    answer.retain(|&x| x != 0);
    answer.sort();
    Ok(answer[answer.len() / 2])
}
