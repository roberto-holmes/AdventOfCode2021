use std::{fs, io};

pub fn run() {
    println!("Day 3");
    println!("Part 1 is {:?}", part1());
    println!("Part 2 is {:?}", part2());
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

fn part2() -> Result<i32, io::Error> {
    // Open file
    let input_str = fs::read_to_string("inputs/day3.txt")?;
    // Extract each line
    let input: Vec<&str> = input_str.split("\n").collect();

    // Calculate how long the bytes are in the input file
    let byte_length = input[0].len();

    // Prepare vectors to store how common 1s and 0s are in each position
    let mut one_counter = Vec::new();
    let mut zero_counter = Vec::new();
    for _ in 0..byte_length {
        one_counter.push(0);
        zero_counter.push(0);
    }

    // Loop through each line of the input file
    for line in &input {
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

    let mut oxygen_valid_bytes = Vec::new();
    let mut co2_valid_bytes = Vec::new();

    let mut single_one_counter;
    let mut single_zero_counter;

    let mut common_oxy = '2';
    let mut common_co2 = '2';

    // Loop through the columns of bits
    for i in 0..byte_length {
        let mut line_number = 0;
        // Loop through each line of the input file
        for line in &input {
            // For the first bit, use the most common value overall
            if i == 0 {
                common_oxy = gamma_bin.chars().nth(i).unwrap();
                common_co2 = common_oxy;
            }

            // Get the bit and determine what it is
            let c = line.chars().nth(i).unwrap();
            // On the first bit, add initial values to vectors. For the rest, remove values
            if i == 0 {
                if c == common_oxy {
                    oxygen_valid_bytes.push(line_number);
                } else {
                    co2_valid_bytes.push(line_number);
                }
            } else {
                if oxygen_valid_bytes.contains(&line_number) && oxygen_valid_bytes.len() != 1 {
                    // Get the position in the vector where the line number is
                    let mut index = 0;
                    if let Some(y) = oxygen_valid_bytes.iter().position(|x| x == &line_number) {
                        index = y;
                    }
                    // Remove values that do not have the most common bit
                    if c != common_oxy {
                        oxygen_valid_bytes.remove(index);
                    }
                }
                if co2_valid_bytes.contains(&line_number) && co2_valid_bytes.len() != 1 {
                    // Get the position in the vector where the line number is
                    let mut index = 0;
                    if let Some(y) = co2_valid_bytes.iter().position(|x| x == &line_number) {
                        index = y;
                    }
                    // Remove values that have the most common bit
                    if c == common_co2 {
                        co2_valid_bytes.remove(index);
                    }
                }
            }
            line_number += 1;
        }
        if i != byte_length - 1 {
            let next_bit = i + 1;
            single_one_counter = 0;
            single_zero_counter = 0;
            // Loop through each value still in the oxygen vector and count 1s and 0s
            for line_index in &oxygen_valid_bytes {
                // Get the bit and determine what it is
                let c = input[*line_index].chars().nth(next_bit).unwrap();
                if c == '1' {
                    single_one_counter += 1;
                } else if c == '0' {
                    single_zero_counter += 1;
                } else {
                    println!("Invalid bit");
                }
            }
            // Find the most common bit
            if single_one_counter >= single_zero_counter {
                common_oxy = '1';
            } else {
                common_oxy = '0';
            }

            // Loop through each value still in the co2 vector and count 1s and 0s
            single_one_counter = 0;
            single_zero_counter = 0;
            for line_index in &co2_valid_bytes {
                // Get the bit and determine what it is
                let c = input[*line_index].chars().nth(next_bit).unwrap();
                if c == '1' {
                    single_one_counter += 1;
                } else if c == '0' {
                    single_zero_counter += 1;
                } else {
                    println!("Invalid bit");
                }
            }
            // Find the most common bit
            if single_one_counter >= single_zero_counter {
                common_co2 = '1';
            } else {
                common_co2 = '0';
            }
        }
    }
    // Use magic to convert from a binary string to a decimal integer
    let oxy_dec = isize::from_str_radix(&input[oxygen_valid_bytes[0]], 2).unwrap();
    let co2_dec = isize::from_str_radix(&input[co2_valid_bytes[0]], 2).unwrap();

    println!(
        "Oxygen bin: {}, dec: {}",
        input[oxygen_valid_bytes[0]], oxy_dec
    );
    println!("CO2 bin: {}, dec: {}", input[co2_valid_bytes[0]], co2_dec);

    // Return the answer once it has been converted to i32
    Ok((oxy_dec * co2_dec).try_into().unwrap())
}
