use std::{fs, io};

pub fn run() {
    println!("Day 3");
    println!("Part 1 is {:?}", part1());
}

fn part1() -> Result<i32, io::Error> {
    // Open file
    let input_str = fs::read_to_string("inputs/day3.txt")?;
    // Extract each line
    let input: Vec<&str> = input_str.split("\n").collect();

    // Calculate how long the bytes are in the input file
    let byte_length = input[0].len();
    println!("{}", byte_length);

    // Prepare vectors to store how common 1s and 0s are in each position
    let mut one_counter = Vec::new();
    let mut zero_counter = Vec::new();
    for _ in 0..byte_length {
        one_counter.push(0);
        zero_counter.push(0);
    }

    // Loop through each line of the input file
    for line in input {
        // Loop through each bit in the byte
        for i in 0..byte_length {
            // Get the bit and determine what it is
            let c = line.chars().nth(i).unwrap();
            if c == '1' {
                one_counter[i] += 1;
            } else if c == '0' {
                zero_counter[i] += 1;
            } else {
                println!("Invalid bit");
            }
        }
    }

    let mut gamma_bin = String::new();
    let mut epsilon_bin = String::new();
    // Determine which bit was the most and least common
    for i in 0..byte_length {
        if one_counter[i] > zero_counter[i] {
            gamma_bin.push('1');
            epsilon_bin.push('0');
        } else if one_counter[i] < zero_counter[i] {
            gamma_bin.push('0');
            epsilon_bin.push('1');
        } else {
            println!("Equal");
        }
    }
    // Use magic to convert from a binary string to a decimal integer
    let gamma_dec = isize::from_str_radix(&gamma_bin, 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&epsilon_bin, 2).unwrap();
    println!("Gamma bin: {}, dec: {}", gamma_bin, gamma_dec);
    println!("Epsilon bin: {}, dec: {}", epsilon_bin, epsilon_dec);

    // Return the answer once it has been converted to i32
    Ok((gamma_dec * epsilon_dec).try_into().unwrap())
}
