use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn part_1(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let regxp = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total_sum = 0;

    for captures in regxp.captures_iter(&input) {
        let first_number = captures.get(1).unwrap().as_str().parse::<i32>()?;
        let second_number = captures.get(2).unwrap().as_str().parse::<i32>()?;

        total_sum += first_number * second_number;
    }

    Ok(total_sum)
}

pub fn part_2(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let regxp = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut total_sum = 0;
    let mut is_mul_enabled = true;

    for captures in regxp.captures_iter(&input) {
        let full_capture = captures.get(0).unwrap().as_str();
        match full_capture {
            "do()" => is_mul_enabled = true,
            "don't()" => is_mul_enabled = false,
            _ => {
                if is_mul_enabled {
                    let first_number = captures.get(1).unwrap().as_str().parse::<i32>()?;
                    let second_number = captures.get(2).unwrap().as_str().parse::<i32>()?;

                    total_sum += first_number * second_number;
                }
            }
        }
    }

    Ok(total_sum)
}
