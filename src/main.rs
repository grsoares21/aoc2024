mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_day_1 = File::open("inputs/1.txt")?;
    let mut reader_day_1 = BufReader::new(file_day_1);

    
    println!("Result day 1 part 1: {}", day_1::part_1(&mut reader_day_1).unwrap());
    println!("Result day 1 part 2: {}", day_1::part_2(&mut reader_day_1).unwrap());

    let file_day_2 = File::open("inputs/2.txt")?;
    let mut reader_day_2 = BufReader::new(file_day_2);

    
    println!("Result day 2 part 1: {}", day_2::part_1(&mut reader_day_2).unwrap());
    println!("Result day 2 part 2: {}", day_2::part_2(&mut reader_day_2).unwrap());

    let file_day_3 = File::open("inputs/3.txt")?;
    let mut reader_day_3 = BufReader::new(file_day_3);

    
    println!("Result day 3 part 1: {}", day_3::part_1(&mut reader_day_3).unwrap());
    println!("Result day 3 part 2: {}", day_3::part_2(&mut reader_day_3).unwrap());

    let file_day_4 = File::open("inputs/4.txt")?;
    let mut reader_day_4 = BufReader::new(file_day_4);

    
    println!("Result day 4 part 1: {}", day_4::part_1(&mut reader_day_4).unwrap());
    println!("Result day 4 part 2: {}", day_4::part_2(&mut reader_day_4).unwrap());

    let file_day_5 = File::open("inputs/5.txt")?;
    let mut reader_day_5 = BufReader::new(file_day_5);

    
    println!("Result day 5 part 1: {}", day_5::part_1(&mut reader_day_5).unwrap());
    println!("Result day 5 part 2: {}", day_5::part_2(&mut reader_day_5).unwrap());

    Ok(())
}
