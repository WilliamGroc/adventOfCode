use std::fs;

fn search_joltage() -> i32 {
    0
}

pub fn run() -> Result<(), std::num::ParseIntError> {
    let contents = read_file();

    let mut counter = 0;

    for lin in contents.lines() {}

    println!("Counter: {}", counter);
    Ok(())
}

fn read_file() -> String {
    fs::read_to_string("./data_day2.txt").expect("Failed to read file")
}
