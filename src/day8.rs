use std::{fs, io};

pub fn run() {
    println!("Day 8");

    // Open file
    let input = fs::read_to_string("inputs/day8.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    println!("Part 2 is {:?}", part2(&input));
}

fn part1(input: &Vec<&str>) -> Result<i32, io::Error> {
    let mut count = 0;
    for line in input {
        let output: Vec<&str> = line.split(" | ").collect();
        let output: Vec<&str> = output[1].split(" ").collect();
        for num in output {
            let seg_count = num.chars().count();
            // Only count numbers 1, 4, 7,and 8
            if seg_count < 5 || seg_count == 7 {
                count += 1
            }
        }
    }
    Ok(count)
}

fn part2(input: &Vec<&str>) -> Result<u32, io::Error> {
    let mut final_display: Vec<u32> = Vec::new();
    for line_num in 0..input.len() {
        let mut numbers = ["0"; 10];
        final_display.push(0);
        let output: Vec<&str> = input[line_num].split(" | ").collect();
        let test_sequence: Vec<&str> = output[0].split(" ").collect();

        // Loop multiple times to ensure all numbers have been found
        for _ in 0..10 {
            for x in &test_sequence {
                let length = x.chars().count();
                // Check for 1
                if length == 2 {
                    numbers[1] = x;
                }
                // Check for 7
                else if length == 3 {
                    numbers[7] = x;
                }
                // Check for 4
                else if length == 4 {
                    numbers[4] = x;
                }
                // Check for 2, 3, and 5
                else if length == 5 {
                    // Check for 3
                    if numbers[3] == "0" {
                        let mut contains_segments = 0;
                        for segment in x.chars() {
                            if numbers[1].contains(segment) {
                                contains_segments += 1;
                            }
                        }
                        if contains_segments == 2 {
                            numbers[3] = x;
                        }
                    }
                    // Check for 5
                    if numbers[6] != "0" && numbers[5] == "0" {
                        let mut contains_segments = 0;
                        for segment in x.chars() {
                            if numbers[6].contains(segment) {
                                contains_segments += 1;
                            }
                        }
                        if contains_segments == 5 {
                            numbers[5] = x;
                        }
                    }
                    // Check for 2
                    if numbers[3] != "0"
                        && numbers[5] != "0"
                        && x != &numbers[3]
                        && x != &numbers[5]
                    {
                        numbers[2] = x;
                    }
                }
                // Check for 0, 6, and 9
                else if length == 6 {
                    // Check for 9
                    if numbers[9] == "0" {
                        let mut contains_segments = 0;
                        for segment in x.chars() {
                            if numbers[3].contains(segment) {
                                contains_segments += 1;
                            }
                        }
                        if contains_segments == 5 {
                            numbers[9] = x;
                        }
                    }
                    // Check for 0
                    if numbers[9] != "0" && x != &numbers[9] && numbers[0] == "0" {
                        let mut contains_segments = 0;
                        for segment in x.chars() {
                            if numbers[7].contains(segment) {
                                contains_segments += 1;
                            }
                        }
                        if contains_segments == 3 {
                            numbers[0] = x;
                        }
                    }
                    // Check for 6
                    if numbers[9] != "0"
                        && numbers[0] != "0"
                        && x != &numbers[0]
                        && x != &numbers[9]
                    {
                        numbers[6] = x;
                    }
                }
                // Check for 8
                else if length == 7 {
                    numbers[8] = x;
                }
            }
        }

        // Get the values that need to be translated
        let output_segments: Vec<&str> = output[1].split(" ").collect();
        for digit in 0..4 {
            // Find what number the display is showing
            let mut current_number = 0;
            // Check each number
            'next_number: for n in 0..10 {
                // Check each segment
                for segment in numbers[n].chars() {
                    // Only keep checking if they share the segment and the same number of segments are lit up
                    if !output_segments[digit].contains(segment)
                        || output_segments[digit].len() != numbers[n].len()
                    {
                        continue 'next_number;
                    }
                }
                current_number = n;
                break;
            }
            final_display[line_num] +=
                current_number as u32 * 10u32.pow((3 - digit).try_into().unwrap());
        }
    }

    // Return sum of all the displayed values
    Ok(final_display.iter().sum())
}
