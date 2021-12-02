use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Window {
    sum: u32,
    count: u8,
}

pub fn run() {
    println!("Day 1");
    println!("Part 1 is {:?}", part1());
    println!("Part 2 is {:?}", part2());
}

fn part1() -> std::io::Result<()> {
    let mut file = File::open("inputs/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut list = contents.lines();
    // Get the first value
    let initial_value_list = list.next();
    let mut initial_value = 0;
    if let Some(j) = initial_value_list {
        // println!("{}", j);
        initial_value = j.trim().parse::<i32>().unwrap();
    }

    // Loop through all the numbers
    let mut last_value = initial_value;
    let mut number_greater = 0;
    for v in list {
        // println!("{} : {}", i, v);
        let current_value = v.parse::<i32>().unwrap();
        if current_value > last_value {
            number_greater += 1;
        }
        last_value = current_value;
    }

    // Print result
    println!("{:#?}", number_greater);
    Ok(())
}

fn part2() -> std::io::Result<()> {
    // Open file and extract contents to list
    // let mut file = File::open("inputs/day1example.txt")?;
    let mut file = File::open("inputs/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let list = contents.lines();
    let list_length = contents.lines().count();
    // Set up vector of windows structs
    let mut win_vec = Vec::with_capacity(4);
    for _ in 0..4 {
        win_vec.push(empty_window());
    }

    // Loop through all the numbers
    let mut i = 0;
    let mut previous_full_window = 0;
    let mut number_greater = 0;
    for v in list {
        // If the window is full
        if win_vec[(i + 1) % 4].count == 3 {
            if win_vec[(i + 1) % 4].sum > previous_full_window && previous_full_window != 0 {
                number_greater += 1;
                // println!("(increased)");
            }
            // else if previous_full_window == 0 {
            //     println!("(n/a)")
            // } else if win_vec[(i + 1) % 4].sum == previous_full_window {
            //     println!("(no change)");
            // } else {
            //     println!("(decreased)");
            // }
            previous_full_window = win_vec[(i + 1) % 4].sum;
            win_vec[i % 4].count = 0;
            win_vec[i % 4].sum = 0;
        }
        win_vec[i % 4].sum += v.parse::<u32>().unwrap();
        win_vec[i % 4].count += 1;
        if i > 0 && i < list_length - 1 {
            win_vec[((i % 4) + 3) % 4].sum += v.parse::<u32>().unwrap();
            win_vec[((i % 4) + 3) % 4].count += 1;
        }
        if i > 1 && i < list_length - 2 {
            win_vec[((i % 4) + 2) % 4].sum += v.parse::<u32>().unwrap();
            win_vec[((i % 4) + 2) % 4].count += 1;
        }
        i += 1;
    }
    // Print result
    println!("Final result is: {}", number_greater + 2);
    Ok(())
}

fn empty_window() -> Window {
    Window { sum: 0, count: 0 }
}
