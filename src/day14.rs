use colored::*;
use std::collections::HashMap;
use std::time::Instant;
use std::{fs, io};

pub fn run() {
    println!("Day 14");

    // Open file
    let input = fs::read_to_string("inputs/day14.txt").expect("File not found");
    // Extract each line
    let input: Vec<&str> = input.split("\r\n").collect();

    println!("Part 1 is {:?}", part1(&input));
    // println!("Part 2 is {:?}", part2(&input));
}

fn part1(input: &Vec<&str>) -> Result<u32, io::Error> {
    let template = input[0];
    let mut rules = HashMap::new();

    for line_num in 2..input.len() {
        let line: Vec<&str> = input[line_num].split(" -> ").collect();
        let new_element: Vec<char> = line[1].chars().collect();
        rules.insert(line[0], new_element[0]);
    }

    let mut polymer: Vec<Vec<char>> = Vec::new();
    let template_chars: Vec<char> = template.chars().collect();

    // Initialise the vector (one vector for the previos polymer and one for the current one)
    for _ in 0..2 {
        polymer.push(Vec::new());
    }

    // Add the template to the vector
    for c in template_chars {
        polymer[0].push(c);
    }

    // println!("Template: {:?}", polymer[0]);

    // Loop through each polymerisation step
    for i in 0..10 {
        // Clear oldest vector for use as next vector
        polymer[(i + 1) % 2].clear();
        // FInd first element and add it to the vector
        let first = polymer[i % 2][0];
        polymer[(i + 1) % 2].push(first);
        // Loop through each element in the previous polymer
        for c_num in 0..polymer[i % 2].len() - 1 {
            let cur = polymer[i % 2][c_num];
            let next = polymer[i % 2][c_num + 1];
            // Get the new element to insert
            let key = cur.to_string() + &next.to_string();
            // Add the new element and the second half of the pair to the new vec
            polymer[(i + 1) % 2].push(*rules.get(&*key).unwrap());
            polymer[(i + 1) % 2].push(next);
        }
    }

    // Store count of elements B,C,H,N
    let mut elements = HashMap::new();

    // If the element exists in the hashmap, insert it, then add 1 regardless
    for c in &polymer[0] {
        let counter = elements.entry(c).or_insert(0);
        *counter += 1;
    }

    // Find the maximum and minimum values in the hashmap
    let mut max = 0;
    let mut min = 0;
    for elem in &elements {
        if min == 0 || min > *elem.1 {
            min = *elem.1;
        }
        if max == 0 || max < *elem.1 {
            max = *elem.1;
        }
    }
    Ok(max - min)
}
