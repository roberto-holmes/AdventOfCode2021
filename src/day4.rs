use std::{fs, io};

#[derive(Debug, Copy, Clone)]
struct Number {
    value: i32,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    rows: [[Number; 5]; 5],
    columns: [[Number; 5]; 5],
    row_marks: [i32; 5],
    column_marks: [i32; 5],
}

#[derive(Debug)]
struct WinningLine {
    is_row: bool,
    is_col: bool,
    index: usize,
}

pub fn run() {
    println!("Day 4");
    println!("Part 1 is {:?}", part1());
    println!("Part 2 is {:?}", part2());
}

fn part1() -> Result<i32, io::Error> {
    // Open file
    let input_str = fs::read_to_string("inputs/day4.txt")?;
    // Extract each line
    let input: Vec<&str> = input_str.split("\n").collect();

    // Separate the drawn numbers
    let drawn_numbers: Vec<&str> = input[0].split(",").collect();

    // Create the vector for storing all the boards
    let mut boards = Vec::new();

    let mut line_count = 0;
    let mut row = 0;
    let mut board_count = 0;
    // Cycle through each line of the input file
    for i in &input {
        // Fudge factor because board 0 starts on line 2
        if line_count == 2 {
            board_count = 0;
        }

        // Ignore drawn numbers and spaces between boards
        if line_count != 0 && *i != "" {
            // Extract the row values
            let mut row_input: Vec<&str> = i.split(" ").collect();
            // Remove extra whitespace
            row_input.retain(|x| *x != "");

            // Add empty board to the vector
            if row == 0 {
                boards.push(Board {
                    rows: [[Number {
                        value: 0,
                        marked: false,
                    }; 5]; 5],
                    columns: [[Number {
                        value: 0,
                        marked: false,
                    }; 5]; 5],
                    row_marks: [0; 5],
                    column_marks: [0; 5],
                })
            }
            // Populate board
            let mut col = 0;
            for val in &row_input {
                boards[board_count].rows[row][col].value = val.parse::<i32>().unwrap_or(-1);
                boards[board_count].columns[col][row].value = val.parse::<i32>().unwrap_or(-1);
                col += 1;
            }
            row += 1;
        }
        // If the board is complete then iterate and reset row
        else {
            row = 0;
            board_count += 1;
        }
        line_count += 1;
    }

    // Store values of the victor
    let mut victorious_board = 0;
    let mut victorious_row = 6;
    let mut victorious_column = 6;
    let mut victorious_number = -2;

    // Loop through each drawn number
    'outer: for i in drawn_numbers {
        // Check each board
        for board_i in 0..boards.len() {
            for row in 0..5 {
                for col in 0..5 {
                    // Check the value in the rows array
                    if boards[board_i].rows[row][col].value == i.parse::<i32>().unwrap_or(-1) {
                        // If it is drawn, mark and count it
                        boards[board_i].rows[row][col].marked = true;
                        boards[board_i].row_marks[row] += 1;
                        // BINGO!!!
                        if boards[board_i].row_marks[row] == 5 {
                            victorious_board = board_i;
                            victorious_row = row;
                            victorious_number = i.parse::<i32>().unwrap_or(-1);
                            break 'outer;
                        }
                    }
                    // Check the value in the columns array
                    if boards[board_i].columns[col][row].value == i.parse::<i32>().unwrap_or(-1) {
                        // If it is drawn, mark and count it
                        boards[board_i].columns[col][row].marked = true;
                        boards[board_i].column_marks[col] += 1;
                        // BINGO!!!
                        if boards[board_i].column_marks[col] == 5 {
                            victorious_board = board_i;
                            victorious_column = col;
                            victorious_number = i.parse::<i32>().unwrap_or(-1);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    print!(
        "Number {} makes board {} is victorious",
        victorious_number, victorious_board
    );
    if victorious_row < 5 {
        println!(" with row {}", victorious_row);
    } else if victorious_column < 5 {
        println!(" with colum {}", victorious_column);
    } else {
        println!(" because of a mistake");
    }

    // println!("{:#?}", boards[victorious_board]);

    // Add up all unmarked numbers on the winning board
    let mut unmarked_sum = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !boards[victorious_board].rows[col][row].marked {
                unmarked_sum += boards[victorious_board].rows[col][row].value;
            }
        }
    }
    println!(
        "The sum is {} and the winning number was {}",
        unmarked_sum, victorious_number
    );
    Ok(unmarked_sum * victorious_number)
}

fn part2() -> Result<i32, io::Error> {
    // Open file
    let input_str = fs::read_to_string("inputs/day4.txt")?;
    // Extract each line
    let input: Vec<&str> = input_str.split("\n").collect();

    // Separate the drawn numbers
    let drawn_numbers: Vec<&str> = input[0].split(",").collect();

    // Create the vector for storing all the boards
    let mut boards = Vec::new();

    let mut line_count = 0;
    let mut row = 0;
    let mut board_count = 0;
    // Cycle through each line of the input file
    for i in &input {
        // Fudge factor because board 0 starts on line 2
        if line_count == 2 {
            board_count = 0;
        }

        // Ignore drawn numbers and spaces between boards
        if line_count != 0 && *i != "" {
            // Extract the row values
            let mut row_input: Vec<&str> = i.split(" ").collect();
            // Remove extra whitespace
            row_input.retain(|x| *x != "");

            // Add empty board to the vector
            if row == 0 {
                boards.push(Board {
                    rows: [[Number {
                        value: 0,
                        marked: false,
                    }; 5]; 5],
                    columns: [[Number {
                        value: 0,
                        marked: false,
                    }; 5]; 5],
                    row_marks: [0; 5],
                    column_marks: [0; 5],
                })
            }
            // Populate board
            let mut col = 0;
            for val in &row_input {
                boards[board_count].rows[row][col].value = val.parse::<i32>().unwrap_or(-1);
                boards[board_count].columns[col][row].value = val.parse::<i32>().unwrap_or(-1);
                col += 1;
            }
            row += 1;
        }
        // If the board is complete then iterate and reset row
        else {
            row = 0;
            board_count += 1;
        }
        line_count += 1;
    }

    // Store values of the victor
    let mut victorious_board = Vec::new();
    let mut victorious_line = Vec::new();
    let mut victorious_number = Vec::new();

    // Loop through each drawn number
    'outer: for i in drawn_numbers {
        // Check each board
        for board_i in 0..boards.len() {
            for row in 0..5 {
                for col in 0..5 {
                    // Check the value in the rows array
                    if boards[board_i].rows[row][col].value == i.parse::<i32>().unwrap_or(-1) {
                        // If it is drawn, mark and count it
                        boards[board_i].rows[row][col].marked = true;
                        boards[board_i].row_marks[row] += 1;
                        // BINGO!!!
                        if boards[board_i].row_marks[row] == 5
                            && !victorious_board.contains(&board_i)
                        {
                            victorious_board.push(board_i);
                            victorious_line.push(WinningLine {
                                is_row: true,
                                is_col: false,
                                index: row,
                            });
                            victorious_number.push(i.parse::<i32>().unwrap_or(-1));
                            if victorious_board.len() == board_count + 1 {
                                break 'outer;
                            }
                        }
                    }
                    // Check the value in the columns array
                    if boards[board_i].columns[col][row].value == i.parse::<i32>().unwrap_or(-1) {
                        // If it is drawn, mark and count it
                        boards[board_i].columns[col][row].marked = true;
                        boards[board_i].column_marks[col] += 1;
                        // BINGO!!!
                        if boards[board_i].column_marks[col] == 5
                            && !victorious_board.contains(&board_i)
                        {
                            victorious_board.push(board_i);
                            victorious_line.push(WinningLine {
                                is_row: false,
                                is_col: true,
                                index: row,
                            });
                            victorious_number.push(i.parse::<i32>().unwrap_or(-1));
                            if victorious_board.len() == board_count + 1 {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
    let last_board_index = victorious_board.len() - 1;
    print!(
        "Number {:?} makes board {:?} the last to win",
        victorious_number[last_board_index], victorious_board[last_board_index]
    );
    if victorious_line[last_board_index].is_row {
        println!(" with row {}", victorious_line[last_board_index].index);
    } else if victorious_line[last_board_index].is_col {
        println!(" with colum {}", victorious_line[last_board_index].index);
    } else {
        println!(" because of a mistake");
    }

    // println!("{:#?}", boards[victorious_board[last_board_index]]);

    // Add up all unmarked numbers on the winning board
    let mut unmarked_sum = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !boards[victorious_board[last_board_index]].rows[col][row].marked {
                unmarked_sum += boards[victorious_board[last_board_index]].rows[col][row].value;
            }
        }
    }
    println!(
        "The sum is {} and the winning number was {}",
        unmarked_sum, victorious_number[last_board_index]
    );
    Ok(unmarked_sum * victorious_number[last_board_index] as i32)
}
