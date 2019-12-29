use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

use failure::{Error, format_err};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_wire_coords(wire_path: &String) -> Result<Vec<Point>, Error> {
    let mut current_point = Point {
        x: 0,
        y: 0,
    };
    let mut wire_coords = Vec::new();
    for x in wire_path.trim().split(",") {
        let (direction, length) = x.split_at(1);
        let length = length.parse::<i32>()?;
        for _ in 0..length {
            let point = match direction {
                "L" => Ok(Point { x: current_point.x - 1, y: current_point.y }),
                "R" => Ok(Point { x: current_point.x + 1, y: current_point.y }),
                "U" => Ok(Point { x: current_point.x, y: current_point.y + 1 }),
                "D" => Ok(Point { x: current_point.x, y: current_point.y - 1 }),
                _ => Err(format_err!("Unknown direction")),
            }?;
            current_point = Point {
                x: point.x,
                y: point.y,
            };
            wire_coords.push(point);
        }
    }
    Ok(wire_coords)
}

fn get_distance(point1: &Point, point2: &Point) -> i32 {
    (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

fn main() -> Result<(), Error> {
    let path = "data/day3.txt";
    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);

    let mut input = String::new();
    buffered.read_line(&mut input)?;
    let wire1: HashSet<Point> = get_wire_coords(&input)?.into_iter().collect();

    let mut input = String::new();
    buffered.read_line(&mut input)?;
    let wire2: HashSet<Point> = get_wire_coords(&input)?.into_iter().collect();

    let origin = Point { x: 0, y: 0};
    let min_distance = wire1.intersection(&wire2)
        .map(|a| get_distance(&a, &origin))
        .min()
        .ok_or(format_err!("Didn't find the min distance. Perhaps there's no intersection."));
    println!("Min Distance = {}", min_distance?);
    Ok(())
}