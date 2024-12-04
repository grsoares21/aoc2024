/** 
 * PART 1 here was probably overkill, I tried to a string for each possible direction and than use Regex
 * in order to search withing each of those strings. I could probably have done something similar to what I did
 * in PART 1 and save some code. But it doesn't matter it helped me to practice some collection manipulations
 */
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn part_1(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let base_character_matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let rows = base_character_matrix.len();
    let cols = base_character_matrix[0].len();

    let mut transposed_character_matrix = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed_character_matrix[j][i] = base_character_matrix[i][j];
        }
    }

    let horizontal_matcher = input.replace('\n', " ");
    let inverted_horizontal_matcher = base_character_matrix
        .iter()
        .map(|line| line.iter().rev().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    let vertical_matcher = transposed_character_matrix
        .iter()
        .map(|line| line.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");
    let inverted_vertical_matcher = transposed_character_matrix
        .iter()
        .map(|line| line.iter().cloned().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    let mut bottom_right_diagonal_vec: Vec<String> = Vec::new();
    for y in (1..base_character_matrix.len()).rev() {
        let mut current_diagonal = String::new();

        let mut current_y = y;
        let mut current_x = 0;
        while current_y < base_character_matrix.len()
            && current_x < base_character_matrix[current_y].len()
        {
            current_diagonal.push(base_character_matrix[current_y][current_x]);
            current_x += 1;
            current_y += 1;
        }

        bottom_right_diagonal_vec.push(current_diagonal);
    }

    for x in 0..cols {
        let mut current_diagonal = String::new();

        let mut current_x = x;
        let mut current_y = 0;
        while current_y < base_character_matrix.len()
            && current_x < base_character_matrix[current_y].len()
        {
            current_diagonal.push(base_character_matrix[current_y][current_x]);
            current_x += 1;
            current_y += 1;
        }

        bottom_right_diagonal_vec.push(current_diagonal);
    }

    let mut top_right_diagonal_vec: Vec<String> = Vec::new();

    for x in (1..cols).rev() {
        let mut current_diagonal = String::new();

        let mut current_x = x;
        let mut current_y = base_character_matrix.len() - 1;
        loop {
            current_diagonal.push(base_character_matrix[current_y][current_x]);

            if current_x + 1 == base_character_matrix[current_y].len() || current_y == 0 {
                break;
            }

            current_x += 1;
            current_y -= 1;
        }

        top_right_diagonal_vec.push(current_diagonal);
    }

    for y in (0..base_character_matrix.len()).rev() {
        let mut current_diagonal = String::new();

        let mut current_y = y;
        let mut current_x = 0;
        loop {
            current_diagonal.push(base_character_matrix[current_y][current_x]);

            if current_x == base_character_matrix[current_y].len() || current_y == 0 {
                break;
            }

            current_x += 1;
            current_y -= 1;
        }

        top_right_diagonal_vec.push(current_diagonal);
    }

    let bottom_right_diagonal_matcher = bottom_right_diagonal_vec.join(" ");
    let top_left_diagonal_matcher = bottom_right_diagonal_vec
        .iter()
        .map(|diagonal| diagonal.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    let top_right_diagonal_matcher = top_right_diagonal_vec.join(" ");
    let bottom_left_diagonal_matcher = top_right_diagonal_vec
        .iter()
        .map(|diagonal| diagonal.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    let complete_matcher = format!(
        "{} {} {} {} {} {} {} {}",
        horizontal_matcher,
        inverted_horizontal_matcher,
        top_left_diagonal_matcher,
        top_right_diagonal_matcher,
        bottom_left_diagonal_matcher,
        bottom_right_diagonal_matcher,
        vertical_matcher,
        inverted_vertical_matcher
    );

    let total = Regex::new(r"XMAS")
        .unwrap()
        .find_iter(&complete_matcher)
        .count() as i32;

    Ok(total)
}

pub fn part_2(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let base_character_matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let rows = base_character_matrix.len();
    let cols = base_character_matrix[0].len();

    let mut x_mas = 0;

    for y in 0..rows - 2 {
        for x in 0..cols - 2 {
            let x_word = format!(
                "{}{}{}{}{}",
                base_character_matrix[y][x],
                base_character_matrix[y][x + 2],
                base_character_matrix[y + 1][x + 1],
                base_character_matrix[y + 2][x],
                base_character_matrix[y + 2][x + 2]
            );

            match x_word.as_str() {
                "MSAMS" | "SMASM" | "MMASS" | "SSAMM" => x_mas +=1,
                _ => ()
            }
        }
    }

    Ok(x_mas)
}
