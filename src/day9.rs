// use colored::*;
use std::{fs, io};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Basin {
    points: Vec<Point>,
    low_point: Point,
}

pub fn run() {
    println!("Day 9");

    // Open file
    let input = fs::read_to_string("inputs/day9.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    println!("Part 2 is {:?}", part2(&input));
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

fn part2(input: &Vec<&str>) -> Result<usize, io::Error> {
    let mut basins: Vec<Basin> = Vec::new();
    let mut basin_count = 0;
    // Loop through y
    for row_num in 0..input.len() {
        let row: Vec<char> = input[row_num].chars().collect();
        // Loop through x
        for col_num in 0..row.len() {
            let current_value = row[col_num].to_digit(10).unwrap();
            let current_point = Point {
                x: col_num,
                y: row_num,
            };
            // If the point is part of a basin
            if current_value != 9 {
                let low_point = find_low_point(input, col_num, row_num);
                if basin_count == 0 {
                    basin_count += 1;
                    // If it isn't, add it
                    basins.push(Basin {
                        points: vec![current_point],
                        low_point,
                    });
                } else {
                    let mut basin_exists = false;
                    // Check through all the basins
                    let mut relevant_basin = 0;
                    for b in 0..basins.len() {
                        // Check if the point is already part of a basin
                        if basins[b].low_point == low_point {
                            basin_exists = true;
                            relevant_basin = b;
                            break;
                        }
                    }
                    if basin_exists {
                        basins[relevant_basin].points.push(current_point);
                    } else {
                        basin_count += 1;
                        // If it doesn't exist, add it
                        basins.push(Basin {
                            points: vec![current_point],
                            low_point,
                        });
                    }
                }
                // print!("{}", current_value.to_string().cyan());
            }
            // if current_value == 9 {
            //     print!("{}", current_value.to_string().red());
            // }
        }
        // println!();
    }
    // Calculate size of each basin in the vector
    let mut basin_size = Vec::new();
    for b in &basins {
        basin_size.push(b.points.len());
    }
    // Get the 3 largest basins
    while basin_size.len() > 3 {
        basin_size.swap_remove(
            basin_size
                .iter()
                .position(|&r| r == *basin_size.iter().min().unwrap())
                .unwrap(),
        );
    }
    // Multiply the sizes together
    let mut answer = 1;
    for i in basin_size {
        answer *= i;
    }
    Ok(answer)
}

// Gradient descent to find the associated low point to any given point on the height map
fn find_low_point(input: &Vec<&str>, col_num: usize, row_num: usize) -> Point {
    let row: Vec<char> = input[row_num].chars().collect();

    let current_value = row[col_num].to_digit(10).unwrap();
    let mut smallest_value = 10;
    // 0-Top, 1-Bot, 2-Left, 3-Right
    let mut smallest_direction = 0;
    // Check each direction
    // Top
    if row_num != 0 {
        smallest_value = input[row_num - 1].chars().collect::<Vec<char>>()[col_num]
            .to_digit(10)
            .unwrap();
        smallest_direction = 0;
    }
    // Bottom
    if row_num != input.len() - 1 {
        let v = input[row_num + 1].chars().collect::<Vec<char>>()[col_num]
            .to_digit(10)
            .unwrap();
        if v < smallest_value {
            smallest_value = v;
            smallest_direction = 1;
        }
    }
    // Left
    if col_num != 0 {
        let v = row[col_num - 1].to_digit(10).unwrap();
        if v < smallest_value {
            smallest_value = v;
            smallest_direction = 2;
        }
    }
    // Right
    if col_num != row.len() - 1 {
        let v = row[col_num + 1].to_digit(10).unwrap();
        if v < smallest_value {
            smallest_value = v;
            smallest_direction = 3;
        }
    }

    // Check if we are at a low point
    if current_value < smallest_value {
        return Point {
            x: col_num,
            y: row_num,
        };
    }
    // If not, run again at the smallest adjacent point
    else {
        let mut new_col = col_num;
        let mut new_row = row_num;
        match smallest_direction {
            3 => new_col += 1,
            2 => new_col -= 1,
            1 => new_row += 1,
            0 => new_row -= 1,
            _ => panic!("Invalid direction"),
        }
        return find_low_point(input, new_col, new_row);
    }
}
