use std::{fs, io};

pub fn run() {
    println!("Day 8");

    // Open file
    let input = fs::read_to_string("inputs/day8.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    // println!("Part 2 is {:?}", part2(&input_num));
}

fn part1(input: &Vec<&str>) -> Result<i32, io::Error> {
    // println!("{:?}", input);
    let mut count = 0;
    for line in input {
        let output: Vec<&str> = line.split(" | ").collect();
        let output: Vec<&str> = output[1].split(" ").collect();
        for num in output {
            let seg_count = num.chars().count();
            if seg_count < 5 || seg_count == 7 {
                count += 1
            }
        }
    }
    Ok(count)
}
