use std::{fs, str};

fn search_joltage(line: &str) -> u32 {
    let mut max_first_number = 0;
    let mut max_second_number = 0;

    let string_length = line.len();

    for (i, c) in line.char_indices() {
        if let Some(current_number) = c.to_digit(10) {
            if string_length - 1 > i && current_number > max_first_number {
                max_first_number = current_number;
                max_second_number = 0;
            } else if (max_first_number * 10 + current_number)
                > (max_first_number * 10 + max_second_number)
            {
                max_second_number = current_number;
            }
        }
    }

    let result = max_first_number * 10 + max_second_number;
    println!(
        "Result for line {}: first: {}, last: {}, result: {}",
        line, max_first_number, max_second_number, result
    );
    result
}

pub fn run() -> Result<(), std::num::ParseIntError> {
    let contents = read_file();

    let mut counter = 0;

    for lin in contents.lines() {
        counter += search_joltage(lin)
    }

    println!("Counter: {}", counter);
    Ok(())
}

fn read_file() -> String {
    fs::read_to_string("./data_day3.txt").expect("Failed to read file")
}
