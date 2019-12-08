use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "data/day1.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut result: u32 = 0;
    for line in buffered.lines() {
        let value = line.unwrap();
        let value = value.parse::<u32>().unwrap();
        let value = (value as f64 / 3.0) as u32;
        result = result + value - 2;
    }

    println!("{}", result);

    Ok(())
}