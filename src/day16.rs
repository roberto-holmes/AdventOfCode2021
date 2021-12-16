use colored::*;
use std::{fs, io};

pub fn run() {
    println!("Day 16");

    // Open file
    let input = fs::read_to_string("inputs/day16.txt").expect("File not found");
    // Extract each line
    println!("Hex: {:?}", input);
    let input = convert_to_binary_from_hex(&input);

    println!("Part 1 is {:?}", part1(&input));
    // println!("Part 2 is {:?}", part2(&input));
}

fn part1(input: &Vec<char>) -> Result<usize, io::Error> {
    println!("{:?}", input);

    let mut pointer = 0;
    let mut version_sum = 0;

    parse_packet(input, &mut pointer, &mut version_sum);

    Ok(version_sum)
}

// Read packet header and decide what to do with the body
fn parse_packet(input: &Vec<char>, pointer: &mut usize, version_sum: &mut usize) -> u64 {
    println!("--------------");
    println!("Parsing packet at pointer: {}", *pointer);
    // First 3 bits are packet version
    let version = convert_to_decimal_from_binary(&input[*pointer..*pointer + 3]);
    // Next 3 bits are type ID
    let type_id = convert_to_decimal_from_binary(&input[*pointer + 3..*pointer + 6]);
    // Move pointer past header
    *pointer += 6;
    println!(
        "Version: {}, Type ID: {}",
        version.to_string().red(),
        type_id.to_string().red()
    );

    // Add version num for Part 1 result
    *version_sum += version as usize;

    let mut result = 0;

    // Literal Value
    if type_id == 4 {
        result = parse_literal_value(input, pointer);
    }
    // Operator
    else {
        // Get the way the subpacket lenghts are stored
        let length_type_id = &input[*pointer];
        println!("Length Type ID: {} ", length_type_id);
        *pointer += 1;
        // Remember where the pointer started
        let initial_pointer = *pointer;
        // Depending on how the subpackets are stored, get their length and advance the pointer
        let mut sub_packet_len = 0;
        let mut sub_packet_count = 0;
        if *length_type_id == '0' {
            sub_packet_len = convert_to_decimal_from_binary(&input[*pointer..*pointer + 15]);
            println!("Subpacket length of {}", sub_packet_len);
            *pointer += 15;
        } else {
            sub_packet_count = convert_to_decimal_from_binary(&input[*pointer..*pointer + 11]);
            println!("Subpacket count of {}", sub_packet_count);
            *pointer += 11;
        }
        // Process each subpacket
        if sub_packet_len != 0 {
            while *pointer - 15 < (sub_packet_len) as usize + initial_pointer {
                parse_packet(input, pointer, version_sum);
            }
        } else if sub_packet_count != 0 {
            for _ in 0..sub_packet_count {
                parse_packet(input, pointer, version_sum);
            }
        } else {
            panic!("Unsure about subpacket lengths");
        }
    }

    result
}

// Get stored numbers in the transmission
fn parse_literal_value(input: &Vec<char>, pointer: &mut usize) -> u64 {
    // Vector for storing binary values
    let mut num = Vec::new();
    // Slice of binary values that will be processed
    let mut group = &input[*pointer..*pointer + 5];
    *pointer += 5;
    loop {
        // println!("{}: {:?}", "Group".green(), group);
        // Add important values to the vector
        for i in 1..5 {
            num.push(group[i]);
        }
        // Check if there are more values to be read
        if group[0] != '1' {
            break;
        }
        // Select the next group of numbers and advance the pointer
        group = &input[*pointer..*pointer + 5];
        *pointer += 5;
    }
    // Get the decimal value
    let result = convert_to_decimal_from_binary(&num);
    println!("{}: {}", "Result".cyan(), result.to_string().red(),);
    result
}

fn convert_to_decimal_from_binary(bin: &[char]) -> u64 {
    let mut dec = 0u64;
    for i in 0..bin.len() {
        dec += bin[i].to_digit(10).unwrap() as u64 * 2u64.pow((bin.len() - i - 1) as u32);
    }
    println!("{} {:?} to {}", "Converted".yellow(), bin, dec);
    dec
}

fn convert_to_binary_from_hex(hex: &str) -> Vec<char> {
    hex.chars()
        .map(to_binary)
        .collect::<String>()
        .chars()
        .collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
