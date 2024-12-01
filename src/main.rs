mod day_1;

use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/1.txt")?;

    let mut reader = BufReader::new(file);

    
    println!("Result day 1 part 1: {}", day_1::part_1(&mut reader).unwrap());
    println!("Result day 1 part 2: {}", day_1::part_2(&mut reader).unwrap());

    Ok(())
}
