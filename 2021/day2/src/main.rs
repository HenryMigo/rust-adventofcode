use std::fs::{self};
use std::io::Result;

fn main() {
    let lines = read_file();
    let data = lines.unwrap();

    part_one(data);
}

fn read_file() -> Result<Vec<String>> {
    let file_name = "day2.txt";
    let result = fs::read_to_string(file_name)?;
    let lines = result.split('\n');
    let mut final_result = Vec::new();
    for line in lines {
        final_result.push(line.to_string());
    }

    return Ok(final_result);
}

fn part_one(data: Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in data {
        let mut to_split = line.split_whitespace();
        let command = to_split.next();
        let amount_string = to_split.next().unwrap();
        let amount: i32 = amount_string.parse().unwrap();

        match command {
            Some("forward") => {
                horizontal = horizontal + amount;
            }
            Some("up") => {
                depth = depth - amount;
            }
            Some("down") => {
                depth = depth + amount;
            }
            Some(&_) => {}
            None => {}
        }
    }

    let total: i32 = depth * horizontal;

    println!("Total: {}", total);
}
