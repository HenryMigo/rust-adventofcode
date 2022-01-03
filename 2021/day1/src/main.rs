use std::fs::{self};
use std::io::Result;

fn main() {
    let lines = read_file();
    let data = lines.unwrap();

    part_one(&data);
    part_two(data);
}

fn read_file() -> Result<Vec<i32>> {
    let file_name = "day1.txt";
    let result = fs::read_to_string(file_name)?;
    let lines = result.split('\n');
    let mut final_result = Vec::new();

    for line in lines {
        let parsed: i32 = line.parse().unwrap();
        final_result.push(parsed);
    }

    return Ok(final_result);
}

fn part_one(lines: &Vec<i32>) {
    let mut previous_measurement = -1;
    let mut count: u32 = 0;

    for (index, line) in lines.iter().enumerate() {
        if previous_measurement == -1 {
            println!("{}. {} (N/A - no previous measurement)", index, line);
        } else {
            if line > &previous_measurement {
                count = count + 1;
                println!("{}. {} (increased)", index, line);
            }

            if line < &previous_measurement {
                println!("{}. {} (decreased)", index, line);
            }
        }
        previous_measurement = line.clone();
    }

    println!("Count: {}", count);
}

fn part_two(lines: Vec<i32>) {
    let mut count: i32 = 0;
    let mut previous_measurement: i32 = -1;

    for (index, line) in (0..lines.len() as i32 - 2).enumerate() {
        let result = lines[index] + lines[index + 1] + lines[index + 2];

        if previous_measurement == -1 {
            println!("{}. {} (N/A - no previous sum)", index, result);
        } else {
            if result > previous_measurement {
                count = count + 1;
                println!("{}. {} (increased)", index, result);
            } else if line < previous_measurement {
                println!("{}. {} (decreased)", index, line)
            }
        }

        previous_measurement = result;
    }

    println!("Count: {}", count);
}
