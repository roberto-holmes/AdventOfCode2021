use std::{cmp, fs, io};

pub fn run() {
    println!("Day 5");
    println!("Part 1 is {:?}", part1());
    println!("Part 2 is {:?}", part2());
}

fn part1() -> Result<i32, io::Error> {
    // Open file
    let input_str = fs::read_to_string("inputs/day5ex.txt")?;
    // Extract each line
    let input: Vec<&str> = input_str.split("\r\n").collect();

    // let mut map = [[0; 1000]; 1000];
    // Don't put it all on the stack to not anger OS
    let mut map = vec![[0; 10]; 10];

    for row in &input {
        let line: Vec<&str> = row.split(" -> ").collect();
        let coord1: Vec<&str> = line[0].split(",").collect();
        let coord2: Vec<&str> = line[1].split(",").collect();
        let x1 = coord1[0].parse::<usize>().unwrap();
        let y1 = coord1[1].parse::<usize>().unwrap();
        let x2 = coord2[0].parse::<usize>().unwrap();
        let y2 = coord2[1].parse::<usize>().unwrap();

        // Vertical Lines
        if x1 == x2 {
            let y_i;
            let y_f;
            if y1 > y2 {
                y_i = y2;
                y_f = y1;
            } else {
                y_i = y1;
                y_f = y2;
            }
            for i in y_i..y_f + 1 {
                map[i][x1] += 1;
            }
        }

        // Horizontal Lines
        if y1 == y2 {
            let x_i;
            let x_f;
            if x1 > x2 {
                x_i = x2;
                x_f = x1;
            } else {
                x_i = x1;
                x_f = x2;
            }
            for i in x_i..x_f + 1 {
                map[y1][i] += 1;
            }
        }
    }

    // Count how many points have overlapping lines
    let mut count = 0;
    for y in map {
        for x in y {
            // if x == 0 {
            //     print!(". ");
            // } else {
            //     print!("{} ", x);
            // }

            if x > 1 {
                count += 1;
            }
        }
        // println!();
    }
    Ok(count)
}

fn part2() -> Result<i32, io::Error> {
    // Open file
    let input_str = fs::read_to_string("inputs/day5ex.txt")?;
    // Extract each line
    let input: Vec<&str> = input_str.split("\r\n").collect();

    // Don't put it all on the stack to not anger OS (10 for ex, 1000 for actual input)
    let mut map = vec![[0; 10]; 10];

    // Loop through lines of the input file
    for row in &input {
        let line: Vec<&str> = row.split(" -> ").collect();
        let coord1: Vec<&str> = line[0].split(",").collect();
        let coord2: Vec<&str> = line[1].split(",").collect();
        let x1 = coord1[0].parse::<usize>().unwrap();
        let y1 = coord1[1].parse::<usize>().unwrap();
        let x2 = coord2[0].parse::<usize>().unwrap();
        let y2 = coord2[1].parse::<usize>().unwrap();

        // Calculate how many points the line has
        let line_length =
            cmp::max((x1 as i32 - x2 as i32).abs(), (y1 as i32 - y2 as i32).abs()) + 1;

        // Go through each point of the line
        for i in 0..line_length as usize {
            let x;
            let y;

            // Calculate the current x
            if x1 == x2 {
                x = x1;
            } else if x1 < x2 {
                x = x1 + i;
            } else {
                x = x1 - i;
            }

            // Calculate the current y
            if y1 == y2 {
                y = y1;
            } else if y1 < y2 {
                y = y1 + i;
            } else {
                y = y1 - i;
            }

            // Add point to map
            map[y][x] += 1;
        }
    }

    let mut count = 0;
    // Go through each point on the map
    for y in map {
        for x in y {
            // Display map
            // if x == 0 {
            //     print!(". ");
            // } else {
            //     print!("{} ", x);
            // }
            // Count points with overlapping lines
            if x > 1 {
                count += 1;
            }
        }
        // println!();
    }
    Ok(count)
}
