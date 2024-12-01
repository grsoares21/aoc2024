use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

pub fn part_1(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;
    
    let mut left_list = vec![];
    let mut right_list = vec![];

    for (index, line) in reader.by_ref().lines().enumerate() {
        match line {
            Ok(content) => {
                let line_tokens: Vec<&str> = content.split_whitespace().collect();

                if line_tokens.len() == 2 {
                    let first_number = line_tokens[0].parse::<i32>()?;
                    let second_number: i32 = line_tokens[1].parse::<i32>()?;

                    left_list.push(first_number);
                    right_list.push(second_number);
                } else {
                    eprintln!(
                        "Error, line {} doesn't contain only 2 values: {}",
                        index, content
                    );
                }
            }
            Err(error) => {
                eprintln!("Error reading line {}: {}", index, error);
            }
        }
    }

    left_list.sort();
    right_list.sort();

    let total_difference = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(first, second)| (first - second).abs())
        .reduce(|a, b| a + b)
        .unwrap();

    Ok(total_difference)
}


pub fn part_2(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut left_list = vec![];
    let mut right_list = vec![];

    for (index, line) in reader.by_ref().lines().enumerate() {
        match line {
            Ok(content) => {
                let line_tokens: Vec<&str> = content.split_whitespace().collect();

                if line_tokens.len() == 2 {
                    let first_number = line_tokens[0].parse::<i32>()?;
                    let second_number: i32 = line_tokens[1].parse::<i32>()?;

                    left_list.push(first_number);
                    right_list.push(second_number);
                } else {
                    eprintln!(
                        "Error, line {} doesn't contain only 2 values: {}",
                        index, content
                    );
                }
            }
            Err(error) => {
                eprintln!("Error reading line {}: {}", index, error);
            }
        }
    }

    let mut similarity_score = 0;

    for left_list_number in left_list {
        for right_list_number in &right_list {
            if left_list_number == *right_list_number {
                similarity_score += left_list_number;
            }
        }
    }

    Ok(similarity_score)
}

