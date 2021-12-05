use std::{fs, io};

pub fn run() {
    println!("Day 4");
    println!("Part 1 is {:?}", part1());
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
