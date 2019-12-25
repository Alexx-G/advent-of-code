use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "data/day2.txt";

    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);
    let mut input = String::new();
    buffered.read_line(&mut input)?;
    let mut program: Vec<u32> = input.split(",")
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();
    program[1] = 12;
    program[2] = 2;
    let mut start: usize = 0;
    loop {
        let op = program[start];
        if op == 1 {
            let i1 = program[start + 1] as usize;
            let i2 = program[start + 2] as usize;
            let result_index = program[start + 3] as usize;
            program[result_index] = program[i1] + program[i2];
        } else if op == 2 {
            let i1 = program[start + 1] as usize;
            let i2 = program[start + 2] as usize;
            let result_index = program[start + 3] as usize;
            program[result_index] = program[i1] * program[i2];
        } else if op == 99 {
            println!("Finised the program - {}", program[0]);
            break;
        }
        start = start + 4;
        if start >= program.len() {
            break;
        }
    }
    Ok(())
}