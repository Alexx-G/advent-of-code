use std::fs::File;
use std::io::{BufReader, BufRead};

use failure::{Error, format_err};

fn init_memory(memory: &mut Vec<usize>, args: &Vec<usize>) {
    memory[1] = args[0];
    memory[2] = args[1];
}

fn execute_opcode(opcode: usize, args: &Vec<usize>) -> Result<usize, Error> {
    match opcode {
        1 => Ok(args[0] + args[1]),
        2 => Ok(args[0] * args[1]),
        _ => Err(format_err!("Unknown opcode {}", opcode)),
    }
}

fn execute_program(memory: &Vec<usize>) -> Result<usize, Error> {
    let mut pointer: usize = 0;
    let mut memory = memory.clone();

    loop {
        if pointer >= memory.len() {
            return Err(format_err!("Didn't find halt opcode"));
        }

        let opcode = memory[pointer];
        if opcode == 99 {
            return Ok(memory[0]);
        }

        let args = vec![memory[memory[pointer + 1]], memory[memory[pointer + 2]]];
        let result = execute_opcode(opcode, &args)?;
        let result_addres = memory[pointer + 3];
        memory[result_addres] = result;
        pointer = pointer + 4;
    }
}

fn main() -> Result<(), Error> {
    let path = "data/day2.txt";

    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);
    let mut input = String::new();
    buffered.read_line(&mut input)?;
    let mut memory: Vec<usize> = input.split(",")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    let initial_args = vec![12, 2];
    init_memory(&mut memory, &initial_args);

    println!("The result is - {}", execute_program(&memory)?);

    for i in 1..99 {
        for j in 1..99 {
            let initial_args = vec![i, j];
            init_memory(&mut memory, &initial_args);
            if execute_program(&memory)? == 19690720 {
                println!("A = {}, B = {}", i, j);
                println!("100 * A + B = {}", 100 * i + j);
            }
        }
    }
    Ok(())
}