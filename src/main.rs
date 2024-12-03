mod day_1;
mod day_2;
mod day_3;

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

    Ok(())
}
