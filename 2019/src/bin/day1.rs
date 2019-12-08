use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn calculate_mass_fuel(mass: &i64) -> i64 {
    let fuel = (*mass as f64 / 3.0).floor() as i64;
    return fuel - 2;
}

fn calculate_fuel(mass: &i64) -> i64 {
    let result = calculate_mass_fuel(mass);
    if result > 0 {
        return result + calculate_fuel(&result);
    } else {
        return 0;
    }
}

fn main() -> Result<(), Error> {
    let path = "data/day1.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let modules_mass: Vec<i64> = buffered.lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    let result: i64 = modules_mass.iter()
        .map(|l| calculate_mass_fuel(l))
        .sum();

    println!("Fuel needed for modules: {}", result);

    let result: i64 = modules_mass.iter()
        .map(|l| calculate_fuel(l))
        .sum();

    println!("Total fuel: {}", result);

    Ok(())
}