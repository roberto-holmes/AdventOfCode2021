use colored::*;
use std::{fs, io};

#[derive(Debug, Copy, Clone)]
struct Octopus {
    energy: u32,
    flashed: bool,
}

impl Octopus {
    fn flash(&mut self) {
        self.flashed = false;
        self.energy = 0;
    }
}

pub fn run() {
    println!("Day 11");

    // Open file
    let input = fs::read_to_string("inputs/day11.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    println!("Part 2 is {:?}", part2(&input));
}

fn part1(input: &Vec<&str>) -> Result<u32, io::Error> {
    // Add input values to array
    let mut grid = [[Octopus {
        energy: 0,
        flashed: false,
    }; 10]; 10];
    for y in 0..input.len() {
        let row: Vec<char> = input[y].chars().collect();
        for x in 0..row.len() {
            grid[x][y].energy = row[x].to_digit(10).unwrap();
        }
    }

    let mut flashes = 0;

    // Number of steps
    for i in 0..100 {
        println!("After step {}", i + 1);
        // Add 1 to all energy levels
        for x in 0..10 {
            for y in 0..10 {
                grid[x][y].energy += 1;
            }
        }
        for _ in 0..10 {
            // Propagate flashes for points with more than 9 energy
            for x in 0..10 {
                for y in 0..10 {
                    propagate(x, y, &mut grid);
                }
            }
        }
        // Calculate flashes
        for y in 0..10 {
            for x in 0..10 {
                if grid[x][y].flashed {
                    flashes += 1;
                    grid[x][y].flash();
                    print!("{}", grid[x][y].energy.to_string().white());
                } else {
                    print!("{}", grid[x][y].energy.to_string().black());
                }
                if grid[x][y].energy > 9 {
                    panic!("{} {}", x, y);
                }
            }
            println!();
        }
        println!();
    }
    Ok(flashes)
}

fn part2(input: &Vec<&str>) -> Result<u32, io::Error> {
    // Add input values to array
    let mut grid = [[Octopus {
        energy: 0,
        flashed: false,
    }; 10]; 10];
    for y in 0..input.len() {
        let row: Vec<char> = input[y].chars().collect();
        for x in 0..row.len() {
            grid[x][y].energy = row[x].to_digit(10).unwrap();
        }
    }

    let mut flashes = 0;
    let mut step = 0;

    // Loop until all octopodes flash (10x10=100)
    while flashes < 100 {
        flashes = 0;
        step += 1;
        println!("After step {}", step);
        // Add 1 to all energy levels
        for x in 0..10 {
            for y in 0..10 {
                grid[x][y].energy += 1;
            }
        }
        for _ in 0..10 {
            // Propagate flashes for points with more than 9 energy
            for x in 0..10 {
                for y in 0..10 {
                    propagate(x, y, &mut grid);
                }
            }
        }
        // Calculate flashes
        for y in 0..10 {
            for x in 0..10 {
                if grid[x][y].flashed {
                    flashes += 1;
                    grid[x][y].flash();
                    print!("{}", grid[x][y].energy.to_string().white());
                } else {
                    print!("{}", grid[x][y].energy.to_string().black());
                }
                if grid[x][y].energy > 9 {
                    panic!("{} {}", x, y);
                }
            }
            println!();
        }
        println!();
    }
    Ok(step)
}

// Propogate flashes to nearby octopodes
fn propagate(x: usize, y: usize, grid: &mut [[Octopus; 10]; 10]) {
    if grid[x][y].energy > 9 && !grid[x][y].flashed {
        grid[x][y].flashed = true;
        // Propagate to points above
        if y > 0 {
            if x > 0 {
                grid[x - 1][y - 1].energy += 1;
                propagate(x - 1, y - 1, grid);
            }
            grid[x][y - 1].energy += 1;
            if x < 9 {
                grid[x + 1][y - 1].energy += 1;
                propagate(x + 1, y - 1, grid);
            }
        }

        // Propagate to points on the same row
        if x > 0 {
            grid[x - 1][y].energy += 1;
            propagate(x - 1, y, grid);
        }
        if x < 9 {
            grid[x + 1][y].energy += 1;
            propagate(x + 1, y, grid);
        }

        // Propagate to points on the row below
        if y < 9 {
            if x > 0 {
                grid[x - 1][y + 1].energy += 1;
                propagate(x - 1, y + 1, grid);
            }
            grid[x][y + 1].energy += 1;
            propagate(x, y + 1, grid);
            if x < 9 {
                grid[x + 1][y + 1].energy += 1;
                propagate(x + 1, y + 1, grid);
            }
        }
    }
}
