/** ATTENTION, THIS CODE DOES NOT REPRESENT MY BELIEFS IN HOW A CODE SHOULD BE STRUCTURED
 * IT WAS DEVELOPED WHILE SLEEPY AND IN A HURRY JUST TO GET TO THE RIGHT ANSWER
 * IT HAS ALL THE WRONG ABSTRACTIONS, TONS OF MUTABILITY AND IS JUST A STRAIGHT MESS
 * MAYBE SOMEDAY IN A HOPEFUL FUTURE I COME BACK TO THIS AND FIX IT WITH SOME BEAUTIFUL RECURSIVE CODE IN A GOOD OLD FP STYLE
*/
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

enum Trend {
    Decreasing,
    Increasing,
    Neither,
}

fn is_report_safe(line_tokens: Vec<&str>) -> Result<bool, Box<dyn std::error::Error>> {
    let (current, rest) = line_tokens.split_first().unwrap();

    let mut current_reading = (*current).parse::<i32>()?;
    let mut is_safe = true;
    let mut current_trend = Trend::Neither;

    for reading in rest {
        let new_reading = (*reading).parse::<i32>()?;

        let difference = (current_reading - new_reading).abs();
        if difference > 3 || difference < 1 {
            is_safe = false;
        }

        match current_trend {
            Trend::Decreasing => {
                if current_reading < new_reading {
                    is_safe = false
                }
            }
            Trend::Increasing => {
                if current_reading > new_reading {
                    is_safe = false
                }
            }
            Trend::Neither => {
                current_trend = if current_reading > new_reading {
                    Trend::Decreasing
                } else {
                    Trend::Increasing
                }
            }
        }

        current_reading = new_reading;
    }

    return Ok(is_safe);
}

pub fn part_1(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut safe_reports = 0;

    for (index, line) in reader.by_ref().lines().enumerate() {
        match line {
            Ok(content) => {
                let line_tokens: Vec<&str> = content.split_whitespace().collect();

                if is_report_safe(line_tokens).unwrap() {
                    safe_reports += 1;
                }
            }
            Err(error) => {
                eprintln!("Error reading line {}: {}", index, error);
            }
        }
    }

    Ok(safe_reports)
}

pub fn part_2(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut safe_reports = 0;

    for (index, line) in reader.by_ref().lines().enumerate() {
        match line {
            Ok(content) => {
                let line_tokens: Vec<&str> = content.split_whitespace().collect();

                let mut is_current_report_safe = false;

                for i in 0..line_tokens.len() {
                    let new_tokens: Vec<&str> = (&line_tokens[..i])
                        .iter()
                        .chain(&line_tokens[i + 1..])
                        .cloned()
                        .collect();

                    if is_report_safe(new_tokens).unwrap() {
                        is_current_report_safe = true;
                        break;
                    }
                }

                if is_report_safe(line_tokens).unwrap() || is_current_report_safe {
                    safe_reports += 1;
                }
            }
            Err(error) => {
                eprintln!("Error reading line {}: {}", index, error);
            }
        }
    }

    Ok(safe_reports)
}
